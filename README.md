# log

- 2022-11-15
  光线加速结构
  scene::sphere_100() 创建了包含100球的cornell box
  不采用bvh的情况下，10线程，各线程20 samples per pixel，最大深度20的情况下渲染耗时10多分钟（还是1000多秒，忘了记了）

  11-15 23:59:实现了简单的bvh，同样的参数，耗时175s

- 2022-11-19
  shape和primitive都返回surfaceinteraction太怪了，本来只是用来存局部几何性质的东西，却用来存了hit_light之类与shape无关的东西

  我希望mesh只是一个几何结构，但是bvh要求里边的东西必须是primitive（这个设计应该是没有问题的，为了让bvh也包含bvh）...

  目前成功地使得一个mesh只被load一次，但是带来的问题是Triangle类需要保存object_to_world矩阵，每一次求交的时候都需要进行三个点的坐标变换，肯定是有更好的方式来减少这一部分计算的

  另外，obj文件里边默认的好像都是右手坐标系，我用的是左手系，load进来后没有做处理的话就导致模型是镜像的了。

  pathintegrator的写法不好看，directintegrator已经很长时间没有改了...

  暂时不要再写新东西了，把上面的问题处理得差不多吧。

  暂时的成果图(ryzen 5600g, 1000spp, 10线程渲染807s)

  ![兔子](bunny_correct.png)