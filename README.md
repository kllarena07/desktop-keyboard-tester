# Chess in Rust with wgpu

## Inspiration

This past Winter 2026 semester I had started getting back into chess after playing it at a coffee chat with a [fellow V1 member](https://www.linkedin.com/in/nikita-nayunipati-193b55276/). I had also been wanting to learn about wgpu, so I thought, with this timing, I might as well try and make chess in wgpu. To restate, the main point of this exercise was to learn wgpu while also remaking a fun game that had recently come back into my life.

## What I Learned

There's two overall pieces to writing wgpu code: CPU code and GPU code.

The GPU code manifests as shader files i.e. `.wgsl` (WGSL is a DSL for shaders) files which contain shader code. A vertex shader is code that will run for each provided vertex that is given to the GPU (a vertex is a point in space with associated data like position, color, texture, coordinates). This shader code runs once for each listed vertex in the code (which is defined on the CPU side for wgpu) and is purely meant to tell the GPU where to place points for an outline of what it is attempting to draw. These vertexes are typically grouped into groups of 3 to form triangles and used to create fragments (the pixels of these triangles.) Then, the fragment shaders tell the GPU how to color the fragments of the triangle. 

The CPU code manifests as in the `renderer.rs` and `vertex.rs` files of this codebase. In these files, you need to setup a communication channel between the CPU and the GPU, through a channel called a `queue`. Through this queue, you to directly upload data to the GPU to be stored in `buffers`, or contiguous blocks of data in memory for data like vertex positions or matrix math. To let your shaders actually use these resources, you group them into `bind groups`, which are just configs that tell the GPU how to access the correct buffers and textures when a `pipeline` is used (we'll get into this below).

When you want the GPU to actually do something, you use a `pipeline` (a list of actions that the GPU will perform when acting on a set of data). To do this, we use something called a `CommandEncoder` to "record" the instructions of the pipeline (you can think of this like watching someone carry out the instructions and then writing it down) to produce a `CommandBuffer`, which is then submitted to the queue and causes the GPU to execute the work.

Now, we need to write CPU and GPU code and setup this queue between them because these two devices do two different things that need to work together to accomplish the task of running this chess game. The CPU is simply in charge of running the program and it's logic (piece movement, what happens to a piece when it's taken) and the GPU is strictly in charge of rendering this game. However, it needs BOTH data from the program and the instructions on how to render everything (buffers and pipelines), so we run the game logic on the CPU side (`app.rs`) and then whenever we need to, write rendering data (`renderer.rs`) back to the GPU to update the drawn state.

(stuff here about surfaces, device, and instance)

_Made with ❤️ by [krayondev](https://x.com/krayondev)_
