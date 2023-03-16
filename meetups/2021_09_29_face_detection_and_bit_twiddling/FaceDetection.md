# Face Detection with Rust

## Who am I

I'm Alex, Software Engineer at Esri with a background in Engineering and Game development domains. 

Can find me on 
- Twitter: @Payne325
- LinkedIn: Alexander Payne
- GitHub: Payne325
- Discord : PaynoInsano


Today I'm going to be talking about a little side project I've been working on which I programmed in Rust and which utilises face detection from live webcam feed. 

## What I set out to do

I've dipped my toe into rust a few times, I've started the rust book, used it in a couple of game jams and used my personal programming kata (text based game) to understand basic I/O. I wanted to make a proof of concept for a computer game that was a bit silly, something I could work on slowly on the side and hopefully a somewhat novel idea.

I thought I'd use Rust to see how feasible it would be to run with such a project. I wasn't sure what I'd end up learning, but was excited to find out. The key really was to pick a project where the keys aim wasn't necessarily to learn Rust, but to see what I picked up along the way
 
What I ended up learning was a little bit about rust language features, and a fair bit about the maturity of computer vision support in the rust crate realm. 

My aim was to make a "Space Invaders" style game that used head tracking to move the player character left and right. I've done a little bit of work with computer vision with Python in the past so I had a rough idea of how such a game would work in mind before starting.

## Why I decided to use OpenCV

I already had prior experience with OpenCV with python, and I knew the underlying library with performant and had multiple ways of doing what I wanted. I was interested in whether there was a rust crate for opencv available, but I also wanted to get an idea of any other applications that I might be able to use. After a bit of digging I arrived at three real options.

- Piston (a modular game engine written in rust) had some imageproc functions, but at the time it was still in development and the roadmap didn't look like it was planning to venture into more serious computer vision. It didn't have quite what I needed and it looks unlikely it ever will. 

- cv-rs - a hand crafted binding to the OpenCV CPP library. Incomplete, and based on commit history the project had been abandoned.

- opencv-rust - automatically generated bindings, requires opencv to be preinstalled, untested.
  - but it did offer almost full support for opencv functions and the ability to control which components of the library are compiled within the toml. 

## Setting up the dev environment

The first step was to download and install the library. For Windows this was recommended via a package manager, I used Chocolatey. For Linux it's recommended for installation via the repository release or to compile from source.

Next I had to set a number of system variables. Most of these were documented but I ran into a few issues with certain system variables missing from the documentation. A number of issues in the repo suggested that this was not an uncommon problem. In fact there were a number of reported problems in the repo that I thankfully didn't see, but the build process certainly seems fragile on both Windows and Linux. 
I couldn't help compare the process to that of opencv for python where you can just install and setup everything via pip in one command.

Final step is to install the libclang-dev or llvm package depending on your OS. The crate depends on these for building. 

## Two main options for detecting faces, Haar Feature classifiers vs Neural Network

- Detect Facial landmarks using HOGs (Histogram of Oriented Gradients)
   - The idea is that feature descriptors can be extracted from an image, and these descriptors are derived from the distribution in intensity gradients within the image.
   - The intensity gradient being a directional change in colour intensity for a neighbourhood of pixels in an image. A descriptor is a concatenation of these.
   - Descriptors are passed through some machine learned model to find the key facial landmarks (using H.O.Gs is fairly agnostic with regards to the model type)
   - I believe that Support-Vector Machines were the first model used when H.O.G. facial detection although I've lost the link to wherever I read that, so citation needed. 

   - Dlib has provided this functionality for many years and is known to work
   - Unofficial rust libraries exist, but this just provides bindings, user has to install dlib
   - I had just come through the opencv set up at this point, so was eager to find a solution that didn't involve more libraries.

- Describe Haar-Feature classifier approach
   - I discovered this after implementing the DNN approach, so did not consider this at the time.
   - A predefined kernel/small matrix called a Haar-feature is applied across an image to detect the likelihood that some facial feature is within a particular subsection/window of the image.
   - Haar-features are used to detect lines and edges where a face would contrast locally between a darker and lighter areas. E.g. eyes vs bridge of the nose.
   - A Haar-feature classifier is a program constructed to group Haar-features into different stages and apply each stage to a window. If a stage fails, then the window is no longer considered for further evaluation. A window that passes all stages contains a facial landmark of some sort.

   - This is very fast, but from my reading it only works well for faces facing directly front.
   - OpenCV provides Haar-cascade Detection via its interface and even provides a weights file within its release files.
   - If I revisit this project, I'd like to experiment with a Harr-feature based implementation of the face tracker. 

- Describe DNN approach
   - I'll admit to some bias for this approach, I had used it before with Python when first experimenting with OpenCV a few years ago
   - For those that don't know, a neural network is a network of layers which uses weights to manipulate some given data. 
   - When training a Neural Net, the result (in our case a classification) is determined as right or wrong, and that information is fed back into the model to adjust the weights at each layer.
   - For face detection, we can feed in the pixel data of an image and have the model return an area supposedly containing the face.

   - OpenCV has a Deep Neural Network (DNN) Module, with an interface for loading a caffe model and weights.
   - It also hides the necessary model and weights for face detection in the source, which is now widely known online.
   - This approach is very fast and doesn't require the user to be directly front facing. 
   - This approach avoids the need to determine/define any facial landmark descriptors, but we can only obtain a bounding box around the face instead of precise detection of the face
      - Acceptable for our purposes   

- Picked NN because
  - prior experience
  - had the model and weights to hand
  - believed it would be more performant when running (best option for a game).

## Implementation

- How it works
   - Runs in a loop, starts by obtaining an image/frame from the camera
      - Currently the program just grabs the first registered camera device, but could be extended to allow for other options.
   - The image is then pre-processed into the size expected by the Neural net and converted to a 'blob'
   - The image blob is passed to the Neural net as input data and a forward pass of the model is triggered. Meaning the image data is passed through the layers
   - The output is returned in the form of a matrix, which we can extract normalised positions within the image to determine a bounding box containing the face.
   - Simply multiply those normalised positions by the original frame width and height to obtain the position in the original image.

   - Some of these points seem like they are quite complicated to do, and it can seem like I've glossed over them here, however each bullet point really only corresponds to one or two opencv functions, the library does a good job of extracting the complicated stuff across all available interfaces.

- How did it go?

- It works! I was able to write a face detection program with Rust and it works quite well.
   - Many of the crate's functions returned values wrapped in Rust's result type, which prompted me to write matches, causing the code to end up looking a bit like try/catch blocks.
   - The C++ opencv lib is known to work well and is quite safe, so the rust binding to it is also performant.
   - For the most part, the functions were a one-to-one mapping with the C++ implementation, made navigating documentation much easier
   - Overall I'd say its extremely feasible to port an OpenCV based application from Python/CPP to Rust. 

- However it wasn't all fun and games, in addition to the difficulty I encountered with the crate set up I also found a few problems during development.
   - Although it was mostly pretty good, the interface was not entirely consistent across the Rust/CPP/Python versions. Particular example, giving an empty string when requesting a layer means all in cpp/python, in rusty it means none. This was not documented.
   - OpenCV's interface has specific types for Input/Output to and from functions, which the auto generated bindings for Rust made somewhat overly complicated when converting these types to traits by obscuring the underlying type. 
   - For example, the OutputVector trait was used for vector and collection of vectors, meaning it got very confusing figuring out how many dimensions I had to deal with in my data source.
   - I also found that end result was a program very coupled to the design of the neural network, which resulted in 
   - Necessary casting of output data
   - Had to hard code the indices for extracting positional info from resultant matrix.
   
   - It's worth noting that these are not necessarily failings of Rust, as these problems would exist with Rust/CPP/Python. The Rust syntax did make these problems very obvious to see.


## Integration into the game

- Created a crate for f-trak to simplify development for myself
  - I could abstract/turn off f-trak in my game on the fly for debugging purposes
  - Admittedly, it's mainly because I had never published a rust crate before and I was curious.
    - The process is well documented and very quick and Chris has given a talk about this before so I won't go into details.

- The Game runs f-trak's function in a separate thread and utilises channels to receive the bounding box containing a face (rust-book chapter 16.2) 

- The Game maps this bounding box to game coordinate system with simple scaling function to convert from camera width/height to game window width/height

- The Player's X position is set to the X position of scaled bounding box centre point, and a game character is drawn on screen.

## References
Thanks for watching/listening/reading.
I've uploaded these slides and some notes to my GitHub, where you can also find the f-trak and bongosero projects as well. 
The notes also contain references to the various articles I read and borrowed images from for this presentation.

https://github.com/not-yet-awesome-rust/not-yet-awesome-rust#computer-vision
https://towardsdatascience.com/whats-the-difference-between-haar-feature-classifiers-and-convolutional-neural-networks-ce6828343aeb
https://docs.opencv.org/3.4/db/d28/tutorial_cascade_classifier.html
https://www.analyticsvidhya.com/blog/2021/07/facial-landmark-detection-simplified-with-opencv/
https://learnopencv.com/facial-landmark-detection/
https://towardsdatascience.com/face-detection-models-which-to-use-and-why-d263e82c302c
https://en.wikipedia.org/wiki/Haar-like_feature
https://en.wikipedia.org/wiki/Histogram_of_oriented_gradients
https://www.eeweb.com/real-time-face-detection-and-recognition-with-svm-and-hog-features/
https://en.wikipedia.org/wiki/Support-vector_machine
https://medium.com/@goutam0157/haar-cascade-classifier-vs-histogram-of-oriented-gradients-hog-6f4373ca239b
https://en.wikipedia.org/wiki/Neural_network