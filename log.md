## 23.6.1

Target: implement RIS-WRS sampler in this renderer

- implement RIS
  - can first check the correctness of RIS using python.  In this case, p can be chosen to be the uniform sampler, and target pdf be the sinusoid signal(or other functions you like). Choose different M (sample candidates) and see if the sampled distribution approximates the target pdf.

  - check the correctness of the WRS algorithm 
  - 
  - implement both of them in this renderer

  - compare the quality with original sampling method (may need some new test scenes). 


- some requisites
  - If I want to test the effectiveness of RIS-WRS sampling like this paper: https://benedikt-bitterli.me/restir/, I need a many-light scene.  The problem is, the renderer does not support scene configuration files (instead use ugly scene creation functions). It will be tiresome to create such a scene. 
  
    So maybe I need to write a .pbrt parser.


## 23.7.16

- continue the previous task
- take the camera out of the integrator struct. The integrator should be isolated from the camera.
- when the light does not hit any object in the scene, it should return a background color, which should also be placed in the scene struct.


## 23.7.23

- complete the direct integrator: support specular transmission and reflection 
- fix path integrator: the sampling seems to be wrong
- enable nee + mis in path integrator 

## 24.1.19
- Now mis is implemented, but not well tested. In order to test it, I need a glossy material, which is not implemented yet. So the next step is to fix the current microfacet implementation, and then test if mis is implemented correctly and excel at reducing noise.

- another problem is that the parser (actually it is not a parser, just a evaluator of the AST, but I do not want to change its name) is incomplete, and its error messages are not friendly to the user. So I may need to change its error handling strategy. Maybe using a crate (such as 'anyhow')?

- after the above 2 tasks, I need to write some test scene files in order to test the current functions of the ray tracer.

- the last is to support rendering the medium, which is difficult for me.