# Chess in Rust with wgpu

## Inspiration

This past Winter 2026 semester I had started getting back into chess after playing it at a coffee chat with a [fellow V1 member](https://www.linkedin.com/in/nikita-nayunipati-193b55276/). I had also been wanting to learn about wgpu, so I thought, with this timing, I might as well try and make chess in wgpu. To restate, the main point of this exercise was to learn wgpu while also remaking a fun game that had recently come back into my life.

## What I Learned

There's two overall pieces to writing wgpu code: CPU code and GPU code.

The GPU code manifests as shader files i.e. `.wgsl` files which contain shader code. A vertex shader is a point in space with associated data (like position, color, texture, coordinates). This shader code runs once for each listed vertex in the code (which is defined on the CPU side for wgpu) and . This code is purely meant to tell the GPU where to place points for an outline of what it is attempting to draw. These vertexes are then broken into fragments of triangles (hence the name; triangles specifically because that is just how gpus draw shapes) which tell the GPU how to shade them in.

The CPU code manifests as in the `renderer.rs` and `vertex.rs` files of this codebase. In these files, you need to setup a communication channel between the CPU and the GPU, through a channel called a `queue`. Through this queue, you then pass in a `pipeline` which are just a list of actions that the GPU will perform when acting on a set of data. The data in this case will be stored as `buffers`, or contiguous blocks of data in memory. Inside of these buffers, we can have data like vertex data (data on where vertices go), textures and their bind groups (textures are images that are being laid out onto triangles and bind groups tell the GPU how to use the image data).

Now, we need to write CPU and GPU code and setup this queue between them because these two devices do two different things that need to work together to accomplish the task of running this chess game. The CPU is simply in charge of running the program and it's logic (piece movement, what happens to a piece when it's taken) and the GPU is strictly in charge of rendering this game. However, it needs BOTH data from the program and the instructions on how to render everything (buffers), so we run the game logic on the CPU side (`app.rs`) and then whenever we need to, write rendering data (`renderer.rs`) back to the GPU to update the drawn state.

_Made with ❤️ by [krayondev](https://x.com/krayondev)_
