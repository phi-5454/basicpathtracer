# A basic path tracer, written in rust.

This is my first implementation of a path tracer. Its architecture is loosely based on ssloy's [tinyraytracer](https://github.com/ssloy/tinyraytracer) and ["Ray tracing in one weekend" (Shirley et. al.)](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
At the moment, it incorporates.

**Observe:** I've made a point to implement as much as possible from scratch. The vector facilities are self-implemented. Indeed, the only external crates used are the rand and rand_dist crates. For simplicity's sake, the output format is chosen to be a .ppm image, which may prove difficult to open for viewing. Some image viewers that support the format are GIMP and feh.

### An example of a render:

![An example render](output.png)
