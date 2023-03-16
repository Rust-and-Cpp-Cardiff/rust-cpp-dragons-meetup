// Type your code here, or load an example.
#include <algorithm>
#include <chrono>
#include <iostream>
#include <thread>
#include <vector>

using namespace std::chrono_literals;

int x = 0;

int main()
{
    auto thread_handles = std::vector<std::thread>();
    for (int i = 0; i < 50000; i++)
    {
        thread_handles.push_back(std::thread([]
                                             { x += 1; }));
    }

    for (auto &t : thread_handles)
    {
        t.join();
    }

    std::cout << "x +=1, loop result: " << x << "\n";
}