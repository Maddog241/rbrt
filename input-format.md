# Input Specification of the Renderer


## Overview

```json

{
    "lookat" : [
        pos.x, pos.y, pos.z,
        look at pos.x, look at pos.y, look at pos.z,
        up vector.x, up vector.y, up vector.z
    ],

    "camera" : {
        "type" : "perspective" or "orthographic",
        "fov" : float
    }, 
    
    "sampler" : {
        "type" : "uniform"
        "count" : float
    },

    "integrator" : {
        "type" : "path" or "direct" or "wrsdirect"
    },

    "film" : {
        "resolution" : [width, height],
        "filename" : "example_scene.png"
    },

    "world" : {
        "lights" : [
            {
                "type": "point" or "area",
                (type specific parameters)
            },
        ],

        "primitives" : [
            {
                "type": "geometric" or "mesh",
                (type specific parameters)
            }
        ]
    }
}

```

## Utilities

```json
"point light" : {
   "pos" : position of the light, 
   "emit" : the radiance it emits,
}

"area light" : {
  "shape" : {
    "disk" :  {
        "pos",
        "up vector",
        "radius"
    },

    "sphere" : {
        "pos",
        "radius"
    },
  },

  "emit"
}

 "geometric primitive" : {
   "shape"
   "material" : {
        "glass" : {
            "eta_a" : refractive index inside,
            "eta_b" : refractive index outside,
            "kr" : relfectance,
            "kt" : transmitance,
        },

        "matte" : {
            "kd" :  diffuse parameter,
        },

        "plastic" : {
            "roughness" : float,
            "ks" : specular,
            "kd" : diffuse
        }
   }
 }

"mesh primitive" : {
    "path" : [
        paths to import the mesh 
        for example "./models/dragon.obj",
    ],

    "material": {

    }
}
 
```