# Rust Nation UK '23

Overall an awesome conference. Very well organized, lots of very friendly
knowledgeable people.

My key takeaways:
- the Rust community is growing fast, and looking for more enterprise engagement
  to help sustain that growth
- The developer background of rust developers is super broad - from low level
  embedded engineers with backgrounds in C, to Go developers from the DevOps
  world, to ReactNative engineers doing app development. There's a wealth of
  talent out there with awesome cross over knowledge with different fields
- lots of folks want to get into Rust. I spoke to quite a few people who are not
  currently working with Rust but part or wholly funded themselves to come to
  the conference to learn and help there chances of moving into this field
- Rust is being used much more in production that I had thought, and the
  projects it has been used for are extremely stable - everyone I have spoken to
  using Rust in their production code has been amazed at the stability of it. We
  can hopefully expect much less time spent debugging our code in Rust!

## Keynote - The Rustacean Cycle: Learn, Teach, Deliver

Nell Shamrell-Harrington delivered a very inspiring keynote highlighting the
extensive effort the rust community put into developing the eco system through
sharing and teaching others. 

The overall aim seemed to be to encourage those attending the conference to
continue engaging with the the larger Rust community, teaching each other,
running meetups, writing blog posts, sharing their experience and asking
questions. ['This week in Rust'](https://this-week-in-rust.org/) was called out
for its unifying of all this new content on Rust being created, giving an entry
point for anyone new to the language and community. 

She also gave a great tip when looking for Rust resources - you can search the
['This week in Rust' github](https://github.com/rust-lang/this-week-in-rust)! I
don't know why I have never thought to do this and definitely will do in the
future!

> _I had a little search for terraform in 'This week in Rust' and found that
> someone has made a tool called [kaws](https://github.com/InQuicker/kaws) for
> creating and managing AWS Kubernetes clusters with Rust via terraform. Nothing
> changed in it since 2017 but still cool to see_ :D

## Stuart Harris - iOS, Android and Web Applications that share a single Rust core

Really interesting talk by [Stuart Harris](https://awesome.red-badger.com/stuartharris/) from consultancy company
[RedBadger](https://red-badger.com/). They've built an open source (but very
much not production ready) cross platform rust back end called
[**crux**](https://github.com/redbadger/crux). It hopes to one day provide an
alternative to frameworks such as Kotlin Multiplatform and Flutter, using Rust.

They have their own md [`Book`](https://redbadger.github.io/crux/) describing
documenting the purpose.

The aim is to move as much business logic and behaviour into a cross platform
rust core (i.e. `crux`), and then have the user interface layer (the shell) be
as thin as possible, built in a platform native language, with declarative UI
frameworks like SwiftUI, Jetpack Compose and React/Vue or a Wasm based framework
(like Yew) on the web.

The end result will in theory be extremely testable, and composable with other
crux built UI components. It felt like a very early doors project, but
definitely one to watch. The architecture is based on Elm which is well loved.

Taken from their wiki:

> A typical message exchange cycle may look like this:
> 
> - User interaction occurs in the Shell, which results in an event
> - The Shell handles this event by constructing an `Event`
> - The Shell calls the Core's `process_event` function passing the `Event` as
>   an argument
> - The Core performs the required processing, updating both its inner state and
>   the view model
> - The Core returns one or more Request messages to the Shell
> 
> In the simplest case, the Core will respond to an Event by returning the
> single Request - `render`.

Although this project is still early doors, I wouldn't be surprised if it
inspires other similar projects - taking the approach of leaving the UI to
platform languages, but still being left with a testable app by moving all the
state handling down into the shared core.

> Aside: their tests ran lightning fast in the demo which they credited to using
> NextTest. This is a cargo executable I'd heard of but not used before. I
> definitely plan to look into it again after seeing it in action!

## Conrad Ludgate - Writing Async from the ground up

An excellent talk by [Conrad Ludgate](https://conradludgate.com/), a developer
at TrueLayer (TrueLayer make an OpenBanking API, it is also where Luca Palmieri
works - the guy who wrote the Zero2Production rust book).

He went through why we need Async, and how you might come up with the same
design if you tried to develop it again today.

### The why:
- OS Threads are by default allowed to context switch, which throws away the
  cache. We can write our own model with a thread per core and pin it
- Makes async control flow easier to manage
- better memory usage (he measured 18kiB per thread vs 2kiB per GoRoutine on his
  mac)
- Works in single threaded environments

So overall aim of async in Rust - allow creation of tasks in the user space
similar to other languages like Go.

To achieve this it makes use of a **Scheduler**.

### Scheduler

- Minimize ready time
- Minimize idle time overall
- Minimizing moving tasks between threads

Two types of scheduling:
- Cooperative scheduling - wait for task to yield. This is a problem if e.g. an
  app crashes and part of it never yields. Could cause an OS crash if enoguh
  threads get locked up this way
- Pre-emptive scheduling - scheduler decides when it thinks a task should stop,
  which may involve interrupting an app mid flow and storing off the state
  somewhere

GoRoutines are pre-emptive. The scheduler moves registers into the stack, checks
if another app/task is waiting to run, if there is one waiting, it starts this
new task. It will pick back up the partially run task at a later time. This
requires the goroutines to have signal handlers.

> I did some googling on this after the talk - from stack overflow: `Since go
> 1.14, the go scheduler is non-cooperative pre-emptive. Each go routine is
> preempted after a certain time slice. It's 10ms in go 1.19.1.` - [GopherCon
> talk on the scheduler](https://www.youtube.com/watch?v=wQpC99Xu1U4)

We can model Cooperative Scheduling as Coroutines:
- Behaves like a big ol' state machine
- Panics can be marked in task by blatting over chunks of the task's state so
  they're not resumed later

### Parking

Conrad introduced the concept of parking - where the OS parks a thread so it
knows it probably won't make progress so should probably be asleep (apparently
there are some subtleties so can't say it's guaranteed not to try to execute).

A simple implementation would be to park ourselves and spawn a separate thread
to unpark us after x time.

In the context of Async, if our queue of `ready` tasks is empty (i.e. the ones
which can make progress), then we sleep the thread with park, and any unparks on
the tasks, which would move them to the `ready` pile, will also wake up the
thread itself.

When you're waiting on an IO task, multiple IO Tasks will be using the same
waker thread, backed by usually linux's kqueue and mac epoll. This is why they
are much better than busy waiting (checking continually on the task).

> More parking doc: https://doc.rust-lang.org/std/thread/fn.park.html

Channels are one way that unpark can be managed. You give the unpark to the
channel, if the channel is already full, then yield, the channel will then be
given execution time, and it will unpark something in its channel to free up
space.

### Futures & Pin

Futures are often self referential. This means they would break if their memory
was ever moved. By default all types implement `Unpin` meaning they can be moved
in memory. For self referential types we MUST use a `Pin` wrapper to stop the
memory moving.

> There's a good explanation of why Futures are often self referential in the
> [Rust Async
> book](https://rust-lang.github.io/async-book/04_pinning/01_chapter.html).

You can pin to both the stack and the heap (but usually need to be more careful
when pinning to the stack). Pinning is done by implementing 

> Creating a Pin type is unsafe, so there are libraries which provide safe
> macros such as
> [`pin_utils`](https://docs.rs/pin-utils/latest/pin_utils/macro.pin_mut.html)
> BUT it's unsafe for a reason - `while we know the pointee of &'a mut T is
> pinned for the lifetime of 'a, we can't know if the data &'a mut T points to
> isn't moved after 'a ends. If it does it will violate the Pin contract.`

We can create a safe abstraction using `Pin<Box<T>>` which won't move, but is a
guaranteed heap allocation.

### Conclusions

There was a lot of good content in this talk, and it's definitely an area I need
to read up on more. He put out a call at the end saying if anyone is interested,
there is a lot still to be shaped in the Async workld of rust (Async traits for
example), so if folks want to get involed they can check out the [Async rust
working group (wg-async) on
Zulip](https://rust-lang.zulipchat.com/#narrow/stream/187312-wg-async).

## Tim McNamara - Spreading Rust to the rest of the company

Tim McNamara wrote `Rust in Action` and works at AWS. He did a great job
describing the growth of the ecosystem, how to bring other into the world of
Rust, and what sort of projects might be fun to use to learn Rust.

He quoted some example of Tilde rewriting their application in Rust, and getting
a 92% reduction in memory (from regularly hitting 100 MB which was their limit
and triggered the service to restart, to consistently staying at 8 MB):

> [How Rust is Tilde’s Competitive
> Advantage](https://www.rust-lang.org/static/pdfs/Rust-Tilde-Whitepaper.pdf)

He also talked about some of the guarantees Rust gives us out of the box, but
also aobut libraries that can take that formal verification further such as
[`Kani`](https://model-checking.github.io/kani/) which can be used to help
verify unsafe rust code.

> Unfortunately `Kani` is unable to verify concurrent code, but the provide a
> [comparison page with other
> tools](https://model-checking.github.io/kani/tool-comparison.html), and for
> concurrency that mentions tokio's [Loom](https://github.com/tokio-rs/loom) and
> AWS Labs' [Shuttle](https://github.com/awslabs/shuttle) as ones that could be
> useful.

Some recommendations Tim offered for getting started with Rust were:
- Write your own rust cheat sheet e.g. Rust concurrency cheat sheet
- Add a snippet to the [Rust
  Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- Write a discord or slack bot

Overall, a good high level overview of why folks are turning to Rust today, and
a few good nuggets of specific details to come away with too.

## Rust Simple GameJam

The game jam was going on throughout the day and I managed to drop in a couple
of times. A couple of folks developing Veloren (a pretty amazing multiplayer
voxel RPG written in Rust and entirely open source) put together a simple game
engine with enough moving parts that attendees could just drop in and move some
stuff around in their game.

It's a really fun and well organized set up so I would definitely recommend
taking a look at their [gamejam
website](https://angelonfira.notion.site/angelonfira/Rust-Nation-Micro-Game-Jam-5f1e931ca1324007abf4acdcc316fe18)
and [mini game engine github](https://github.com/AngelOnFira/micro-jam-engine).

## Moving beyond `Arc<Mutex<T>>`

Katharina Fey ran this talk on how to think about sharing memory, and ways we
can do it better.

### Basics of locking & atomics

The talk started with a general discussion on data races, and simple ways they
are overcome, such as the mutual exclusion principle, using a 'Signal first'
approach to ensure no two threads ever access the same memory at the same time.
Worst case, neither get the memory, and the OS has mechanisms in place to
stagger subsequent attempts to get the memory to avoid live locks.

They then moved on to a whistle stop tour of atomics:
- Synchronizes memory between writers and readers
- Uses a cache coherency protocol (e.g.
  [mesi](https://en.wikipedia.org/wiki/MESI_protocol))
- In rust, the compare exchange operation takes:
  - Expected previous value to prove you're first
  - New value to set
  - Ordering to use if set succeeds
  - Ordering to use if set fails
- Ordering determines how much the compiler or OS can reorder your operations as
  seen on different threads e.g. `Ordering::Release` means all atomic loads
  **before** an acquire will be observable.

### Locking smarter

A simple example of locking better might be locking subsections of a hashmap
e.g. by by using a superindex of the first 5 chars of the hash.

We could also use copy-on-write, where every write creates a new node, meaning
old readers still have access to the old data, and then hazard pointers can be
used to keep track of nodes still being read

Alternatively, 'epoch-reclamation' can be used, which similarly contains s list
of notes to delete, but per epoch.

`crossbeam_epoch` uses epoch based reclamation.


### Additional tidbits

- If using `compare_exachnage` in a loop, consider whether
  `compare_exchange_Weak` should be used in stead, since `compare_exchange` is
  essentially a hardware loop already so you really have nested loops otherwise
- If you don't need shareable state, just producer/consumer, then preferable to
  use channels
- A questions was asked about how this related to the idea of 'Eventually
  consistent systems', to which they answered that that's how network
  distrubuted systems work, and relies on consensus algorithms.

### Take aways

A good overall summary of atomics and mutexes. Nothing super new but good
reinforcement.

## The Rust Foundation Q&A

Really cool hearing some of the aims of the rust foundation, and how much
they've achieved in a year. Represented were **Rebecca Rumbul** - Executive
Director (and based in Cardiff!), **Joel Marcey** - Director of Technology (who
I also spoke to earlier in the conference and hopefully he's going to send me
some resources on security as they get created), **Stephen Chin** - member
director from JFrog, and **Paul Lenz** - Director of Finance and Funding.

They have:
- a new security staff member - who will try to sure up the security of the rust
  toolchain and help gives companies the sort of verifiability they need
- focussed their efforts on supporting the Rust project, but may broaden their
  aims in the future
- reinforced support for the build system - which builds on donated
  infrastructure, hosting crates.io and the whole build system

Additional things they do:
- protecting open source maintainers from liability, providing legal advice
- managing money:
  - funding more people to work on the project
  - supporting education and training materials
  - managing infrastructure growth costs so they can ensure long term success of
    the Rust project
- trying to get governments and companies to support open source and see the
  value of it

They also run a grants program:
- $1000 a month stipend for fellowship grant holders
- project grants for specific pieces of work
- hardship grants for ecosystem contributors
- small community grants for things like local meetups

Key takeaways: they're doing awesome work - managing just a fast growing
ecosystem is not easy and they're doing a great job! I would love to see Esri
represented at the Rust Foundation.

## Closing KeyNote - Jon Gjengset

Excellent closing keynotes. Personally my favourite talk of the conference. The
Tldr;s of this talk were:
- Stay on the latest version of the rust compiler if at all possible to save
  headaches later. Even if it means ignoring warnings for a while
- Work on up to date OSs. The rust project tries to support old OSs, but if the
  tooling they rely goes out of support then there's not a lot they can do
- Feed back things upstream - if a crate brakes for you, feed it back to the
  crate owner - it may be that it was an accident! Feed it back and they might
  be able to roll it back and make it a breaking version change instead.

### On updating compiler version & managing warnings

Jon talked about the different types of rust versioning, what editions mean
(isolated at crate boundaries and supported by future rust compilers too), and
topically for us, how to manage upgrading.

So far we've been able to update pretty frequently update the rust compiler
version, and just fix the warnings as they come. Going forwards, the number of
warning to tackle might become too much and delay upgrading the version, since
you can't merge a PR when there are clippy warnings. Jon's recommendation is to
NOT treat warning as errors, and instead have a regular separate process for
fixing warnings. That way you should always be able to upgrade.

This is important because in Rust, requiring an updated compiler version is not
considered a breaking change, meaning it could break from underneath us quite
easily.

### On nightly & unsafe

His recommendation is to not use nightly rust compiler in production since it
breaks a lot and the actual use cases for it are rare. His one exception is if
by using nightly you can reduce the number of the pieces on unsafe code in the
codebase (We have none currently so not worries our side :)).

### On updating crate versions

He would recommend:
- Automate cargo update process
- Use tools like `cargo sem-ver-checks` and `cargo public-api` to look for
  accidental breaking changes
- Use `cargo outdated` to look for newer dependencies beyond just the one
  specified in the toml, since older versions are in reality rarely updated

Additional thoughts: wouldn't it be nice if there was a way to signify long term
version support on crates

### On crate abandonment

Crates getting abandoned can be a real problem. He recommends to be selective
when choosing crates:
- Are there multiple people committing to the repo?
- Are there multiple owners
- what's the ratio of out-standing PR's to closed PRs? 
- What's the latest GitHub activity (latest commit, commit frequency, etc.)

And to be proactive:
- fund developers of crates you use
- contribute!

## Post-Conference RedBadger Enterprise brunch

The post conference brunch had a lot of really keen people who either work with
rust, or want to be, brainstorming how to better introduce rust to enterprise
work places. My key takeaways were:
- We need failure stories as well as success ones, someone shared some stories
  of a project he worked on at google, which didn't fail, but also didn't
  thrive, other developers in the department stayed with their current preferred
  languages and the rust developers working on it moved on
- We need to be loud about where Rust is getting used. It's quietly growing in
  lots of companies, but we only really hear about the big names
- We need better learning resources that take you beyond beginner. A proper
  learning track for different domains

## Other comments

The conference was great beyond just these talks, chats at the sponsor area with
developers and rust foundation members were invaluable (I even got chatting to
someone who works on ElastiCache!). The takeaway for me is that there is a whole
network of people keen to help each other, and we should definitely try to tap
into thar and contribute back into it.

[Full writeup including the day1 workshop is in a repo on my account](https://devtopia.esri.com/ciar8927/RustNationUK_2023)