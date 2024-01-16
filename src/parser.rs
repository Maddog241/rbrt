use core::panic;
use std::{fs, sync::Arc};
use cgmath::{Vector3, Vector4, Point2, InnerSpace};
use json::JsonValue;

use crate::{scene::Scene, camera::{perspective::PerspectiveCamera, film::Film}, geometry::{transform::Transform, shape::{Shape, disk::Disk, sphere::Sphere}}, WorldSetting, integrator::{path_integrator::PathIntegrator, direct_integrator::DirectIntegrator, Integrator}, light::{LightList, Light, area::AreaLight}, accelerator::bvh::BVH, primitive::{geometric_primitive::GeometricPrimitive, Primitive}, material::{Material, matte::Matte, plastic::Plastic, glass::Glass, mirror::Mirror}, spectrum::Spectrum, texture::constant::ConstantTexture, mesh::TriangleMesh, sampler::uniform_sampler::UniformSampler};

macro_rules! report_parsing_error {
    ($s:expr) => {
        {
            let mut m = String::from("SCENE PARSING FAILED: ");
            m.push_str($s);
            panic!(stringify!(m))
        }
    };
}

fn get_object_property(value: JsonValue, property: &str) -> JsonValue {
    if let JsonValue::Object(obj) = value {
        match obj.get(property) {
            Some(value) => value.clone(),
            None => panic!("JsonValue does not have property {}", property),
        }
    } else {
        panic!("value is not an object")
    }
}

fn parse_number(number: JsonValue, debug_info: &str) -> f64 {
    match number {
        JsonValue::Number(num) => num.into(),
        _ => report_parsing_error!(debug_info),
    }
}

fn parse_vec3(scale: &Vec<JsonValue>, name: &str) -> Vector3<f64> {
    if scale.len() != 3 {
        let msg = format!("the size vector {} should be 3", name);
        report_parsing_error!(msg.as_str());
    }

    match scale[0..3] {
        [JsonValue::Number(x), JsonValue::Number(y), JsonValue::Number(z)] => {
            Vector3::new(x.into(), y.into(), z.into())
        }
        _ => panic!("SCENE PARSING ERROR: vector {}'s elements are not of type 'Number'", name)
    }
}

fn parse_rotate(scale: &Vec<JsonValue>, name: &str) -> Vector4<f64> {
    if scale.len() != 4 {
        let msg = format!("the size vector {} should be 3", name);
        report_parsing_error!(msg.as_str());
    }

    match scale[0..4] {
        [JsonValue::Number(x), JsonValue::Number(y), JsonValue::Number(z), JsonValue::Number(w)] => {
            let v = Vector4::new(x.into(), y.into(), z.into(), w.into());
            if v.truncate().magnitude2() == 0.0 {
                eprintln!("warning: the rotate axis should not be (0, 0, 0), using (1, 0, 0) instead");
                Vector4::new(1.0, 0.0, 0.0, 0.0)
            } else {
                v
            }
        }
        _ => panic!("SCENE PARSING ERROR: vector {}'s elements are not of type 'Number'", name)
    }
}

fn parse_shape(shape: JsonValue) -> Box<dyn Shape> {
    let tp = get_object_property(shape.clone(), "type");
    match tp {
        JsonValue::Short(tp) => {
            match tp.as_str() {
                "disk" => parse_disk(shape.clone()),
                "sphere" => parse_sphere(shape),
                // "cylinder" => parse_cylinder(shape),
                _ => panic!(""),
            }
        },
        _ => panic!(),
    }
}

fn parse_disk(shape: JsonValue) -> Box<dyn Shape> {
    if let JsonValue::Object(shape) = shape {
        // radius
        let radius: f64 = match shape.get("radius") {
            Some(JsonValue::Number(radius)) => radius.clone().into(),
            _ => panic!(),
        };
        // scale
        let scale = match shape.get("scale") {
            Some(JsonValue::Array(scale)) => {
                parse_vec3(scale, "scale")
            },
            _ => panic!(),
        };
        // rotate
        let rotate = match shape.get("rotate") {
            Some(JsonValue::Array(rotate)) => {
                parse_rotate(rotate, "rotate")
            },
            _ => panic!(),
        };

        // shift
        let trans = match shape.get("translate") {
            Some(JsonValue::Array(shift)) => {
                parse_vec3(shift, "translate")
            },
            _ => panic!(),
        };

        let object_to_world = 
            Transform::translate(trans) * 
            Transform::rotate(rotate.w, rotate.truncate().normalize()) * 
            Transform::scale(scale);
        
        let world_to_object = object_to_world.inverse();

        Box::new(Disk::new(object_to_world, world_to_object, radius))
    } else {
        panic!()
    }
}

fn parse_sphere(shape: JsonValue) -> Box<dyn Shape> {
    if let JsonValue::Object(shape) = shape {
        // radius
        let radius: f64 = match shape.get("radius") {
            Some(JsonValue::Number(radius)) => radius.clone().into(),
            _ => panic!(),
        };
        // scale
        let scale = match shape.get("scale") {
            Some(JsonValue::Array(scale)) => {
                parse_vec3(scale, "scale")
            },
            _ => panic!(),
        };
        // rotate
        let rotate = match shape.get("rotate") {
            Some(JsonValue::Array(rotate)) => {
                parse_rotate(rotate, "rotate")
            },
            _ => panic!(),
        };

        // shift
        let trans = match shape.get("translate") {
            Some(JsonValue::Array(shift)) => {
                parse_vec3(shift, "translate")
            },
            _ => panic!(),
        };

        let object_to_world = 
            Transform::translate(trans) * 
            Transform::rotate(rotate.w, rotate.truncate().normalize()) * 
            Transform::scale(scale);
        
        let world_to_object = object_to_world.inverse();

        Box::new(Sphere::new(object_to_world, world_to_object, radius))
    } else {
        panic!()
    }
}

fn parse_material(mat: JsonValue) -> Arc<dyn Material> {
    let tp = get_object_property(mat.clone(), "type");
    match tp {
        JsonValue::Short(tp) => {
            match tp.as_str() {
                "matte" => parse_matte(mat.clone()),
                "plastic" => parse_plastic(mat.clone()),
                "glass" => parse_glass(mat.clone()),
                "mirror" => parse_mirror(mat),
                _ => {
                    let msg = format!("no material type named {}", tp);
                    report_parsing_error!(msg.as_str());
                },
            }
        },
        _ => report_parsing_error!("type should be a string"),
    }
}

fn parse_matte(mat: JsonValue) -> Arc<dyn Material> {
    if let JsonValue::Object(mat) = mat {
        let kd = match mat.get("kd") {
            Some(JsonValue::Array(kd)) => {
                parse_vec3(kd, "kd")
            },

            // Some(JsonValue::Object(kd)) => {
            //     match kd.get("type") {
            //     }
            // }
            _ => panic!(),
        };

        let kd = Spectrum::new(kd.x, kd.y, kd.z);

        Arc::new(Matte::new(Box::new(ConstantTexture::new(kd))))
    } else {
        panic!()
    }
}

fn parse_plastic(mat: JsonValue) -> Arc<dyn Material> {
    // roughness
    if let JsonValue::Object(mat) = mat {
        let roughness = match mat.get("roughness") {
            Some(JsonValue::Number(roughness)) => roughness.clone().into(),
            _ => report_parsing_error!("material's roughness should be a number"),
        };

        let ks = match mat.get("ks") {
            Some(JsonValue::Array(ks)) => {
                parse_vec3(ks, "ks")
            },
            _ => report_parsing_error!("plastic's ks property should be of array type"),
        };

        let kd = match mat.get("kd") {
            Some(JsonValue::Array(kd)) => {
                parse_vec3(kd, "kd")
            },
            _ => report_parsing_error!("plastic's ks property should be of array type"),
        };

        let ks = Spectrum::new(ks.x, ks.y, ks.z);
        let kd = Spectrum::new(kd.x, kd.y, kd.z);

        Arc::new(Plastic::new(roughness, ks, kd))
    } else {
        panic!()
    }
}

fn parse_glass(mat: JsonValue) -> Arc<dyn Material> {
    if let JsonValue::Object(mat) = mat {
        let eta_a = match mat.get("eta_a") {
            Some(JsonValue::Number(eta_a)) => eta_a.clone().into(),
            _ => report_parsing_error!("glass's eta_a should be a number"),
        };

        let eta_b = match mat.get("eta_b") {
            Some(JsonValue::Number(eta_b)) => eta_b.clone().into(),
            _ => report_parsing_error!("glass's eta_b should be a number"),
        };

        let kr = match mat.get("kr") {
            Some(JsonValue::Array(kr)) => {
                parse_vec3(kr, "kr")
            },
            _ => report_parsing_error!("glass's ks property should be of array type"),
        };

        let kt = match mat.get("kt") {
            Some(JsonValue::Array(kt)) => {
                parse_vec3(kt, "kt")
            },
            _ => report_parsing_error!("glass's ks property should be of array type"),
        };

        let kr = Spectrum::new(kr.x, kr.y, kr.z);
        let kt = Spectrum::new(kt.x, kt.y, kt.z);

        Arc::new(Glass::new(eta_a, eta_b, kr, kt))
    } else {
        panic!()
    }
}

fn parse_mirror(mat: JsonValue) -> Arc<dyn Material> {
    if let JsonValue::Object(mat) = mat {
        let r = match mat.get("reflectance") {
            Some(JsonValue::Array(reflectance)) => {
                parse_vec3(reflectance, "reflectance")
            },
            _ => report_parsing_error!("mirror's reflectance property should be of array type"),
        };

        Arc::new(Mirror::new(Spectrum::new(r.x, r.y, r.z)))
    } else {
        panic!()
    }
}


fn parse_primitive(primi: JsonValue) -> Box<dyn Primitive> {
    let tp = get_object_property(primi.clone(), "type");
    match tp {
        JsonValue::Short(tp) => {
            match tp.as_str() {
                "geometric" => parse_geometric(primi),
                // "mesh" => parse_mesh(primi),
                _ => {
                    let msg = format!("no primitive type named {}", tp);
                    report_parsing_error!(msg.as_str());
                },
            }
        }
        _ => report_parsing_error!("type should be a string"),
    }
}

fn parse_geometric(primi: JsonValue) -> Box<dyn Primitive> {
    let shape = get_object_property(primi.clone(), "shape");
    let shape = parse_shape(shape);
    
    let mat = get_object_property(primi, "material");
    let mat = parse_material(mat);

    Box::new(GeometricPrimitive::new(shape, mat))
}

fn parse_string(s: JsonValue) -> String {
    match s {
        JsonValue::Short(s) => String::from(s.as_str()),
        JsonValue::String(s) => s,
        _ => report_parsing_error!("mesh's path should be a string"),
    }
}

// fn parse_mesh(primi: JsonValue) -> Box<dyn Primitive> {
//     let path = get_object_property(primi, "path");
//     let path = parse_string(path);
//     let mesh = TriangleMesh::load(&path);
// }

fn parse_light(light: JsonValue) -> Arc<dyn Light> {
    let tp = get_object_property(light.clone(), "type");
    match tp {
        JsonValue::Short(tp) => {
            match tp.as_str() {
                "area" => parse_area(light.clone()),
                "point" => parse_point(light),
                _ => {
                    let msg = format!("no light type named {}", tp);
                    report_parsing_error!(msg.as_str());
                },
            }
        }
        _ => report_parsing_error!("type should be a string"),
    }
}

fn parse_area(light: JsonValue) -> Arc<dyn Light> {
    let shape = get_object_property(light.clone(), "shape");
    let emit = get_object_property(light, "emit");
    if let JsonValue::Array(emit) = emit {
        let emit = parse_vec3(&emit, "emit");
        let shape = parse_shape(shape);
        Arc::new(AreaLight::new(shape, Spectrum::new(emit.x, emit.y, emit.z)))
    } else {
        report_parsing_error!("area light's emit should be an vec3");
    }
}

fn parse_point(_light: JsonValue) -> Arc<dyn Light> {
    unimplemented!()
}

fn parse_world(world: JsonValue) -> Scene {
    let json_lights = get_object_property(world.clone(), "lights");
    let json_primitives = get_object_property(world, "primitives");

    let mut lights = Vec::new();
    let mut primitives = Vec::new();
    
    match json_lights {
        JsonValue::Array(json_lights) => {
            for json_light in json_lights {
                lights.push(parse_light(json_light));
            }
        },
        _ => report_parsing_error!("'lights' should be of array type"),
    }

    match json_primitives {
        JsonValue::Array(json_primitives) => {
            for json_primi in json_primitives {
                primitives.push(parse_primitive(json_primi));
            }
        },
        _ => report_parsing_error!("'primitives' should be of array type"),
    }

    let lightlist = LightList::new(lights);
    let bvh = BVH::new(primitives);

    Scene::new(lightlist, Box::new(bvh))
}

// fn parse_cylinder(shape: JsonValue) -> Box<dyn Shape> {
//     if let JsonValue::Object(shape) = shape {

//     }
// }

fn parse_camera(camera: JsonValue) -> PerspectiveCamera {
    let tp = get_object_property(camera.clone(), "type");
    if let JsonValue::Short(tp) = tp {
        match tp.as_str() {
            "perspective" => parse_perspective(camera),
            _ => {
                let msg = format!("no type {} for camera", tp);
                report_parsing_error!(msg.as_str());
            },
        }
    } else {
        report_parsing_error!("camera type should be a string");
    }
}

fn parse_perspective(camera: JsonValue) -> PerspectiveCamera {
    let lookat = get_object_property(camera.clone(), "lookat");
    let film = get_object_property(camera.clone(), "film");
    let fov = get_object_property(camera, "fov");

    // lookat 
    let camera_to_world = match lookat {
        JsonValue::Array(lookat) => {
            assert!(lookat.len() >= 9);
            let pos = match lookat[0..3] {
                [JsonValue::Number(x), JsonValue::Number(y), JsonValue::Number(z)] => Vector3::new(x.into(), y.into(), z.into()),
                _ => report_parsing_error!("lookat matrix should only contain numbers"),
            };

            let lookat_pos = match lookat[3..6] {
                [JsonValue::Number(x), JsonValue::Number(y), JsonValue::Number(z)] => Vector3::new(x.into(), y.into(), z.into()),
                _ => report_parsing_error!("lookat matrix should only contain numbers"),
            };

            let upvector = match lookat[6..9] {
                [JsonValue::Number(x), JsonValue::Number(y), JsonValue::Number(z)] => Vector3::new(x.into(), y.into(), z.into()),
                _ => report_parsing_error!("lookat matrix should only contain numbers"),
            };

            Transform::look_at(pos, lookat_pos, upvector.normalize()).inverse()
        },

        _ => report_parsing_error!("lookat matrix should be an array"),
    };

    // film 
    let resolution = get_object_property(film.clone(), "resolution");
    let filename = get_object_property(film, "filename");

    let (width, height): (f64, f64) = match resolution {
        JsonValue::Array(res) => {
            assert!(res.len() >= 2);
            match res[0..2] {
                [JsonValue::Number(width), JsonValue::Number(height)] => (width.into(), height.into()),
                _ => report_parsing_error!("resolution should contain 2 numbers"),
            }
        },
        _ => report_parsing_error!("resolution should be an array"),
    };

    let filename = parse_string(filename);
    let film = Film::new(&filename, width as usize, height as usize);

    // fov
    let fov = parse_number(fov, "fov should be a number");

    let frame: f64 = width / height;

    PerspectiveCamera::new(
        camera_to_world,
        (Point2::new(-frame, -1.0), Point2::new(frame, 1.0)),
        0.0,
        1.0,
        fov,
        film
    )
}

fn parse_setting(setting: JsonValue) -> WorldSetting {
    let n_sample = get_object_property(setting.clone(), "n_sample");
    let n_thread = get_object_property(setting.clone(), "n_thread");
    let sampler = get_object_property(setting.clone(), "sampler");
    let integrator = get_object_property(setting, "integrator");


    let n_sample = parse_number(n_sample, "spp should be a number").max(1.0) as usize;
    let n_thread = parse_number(n_thread, "n_thread should be a number").max(1.0) as usize;

    // sampler
    let sampler_tp = get_object_property(sampler, "type");
    let sampler = match sampler_tp {
        JsonValue::Short(tp) => {
            match tp.as_str() {
                "uniform" => {
                    UniformSampler::new()
                },
                _ => {
                    let msg = format!("no type {} for sampler", tp);
                    report_parsing_error!(msg.as_str());
                },
            }
        },
        _ => report_parsing_error!("sampler type should be a string"),
    };

    // integrator
    let integrator_tp = get_object_property(integrator, "type");
    let integrator: Arc<Box<dyn Integrator>>= match integrator_tp {
        JsonValue::Short(tp) => {
            match tp.as_str() {
                "path" => Arc::new(Box::new(PathIntegrator::new(20))),
                "direct" => Arc::new(Box::new(DirectIntegrator::new(20))),
                // "wrsdirect" => setting.integrator = Box::new,
                _ => {
                    let msg = format!("no type {} for integrator", tp);
                    report_parsing_error!(msg.as_str());
                }
            }
        },
        _ => report_parsing_error!("sampler type should be a string"),
    };

    WorldSetting::new(
        n_sample,
        n_thread,
        integrator,
        Arc::new(sampler),
    )
}



pub fn parse_scene(path: &str) -> (WorldSetting, PerspectiveCamera, Scene) {
    let file_soure = fs::read_to_string(path).unwrap();

    match json::parse(&file_soure) {
        Ok(config) => {
            let camera = get_object_property(config.clone(), "camera");
            let setting = get_object_property(config.clone(), "setting");
            let world = get_object_property(config, "world");

            // world
            let camera = parse_camera(camera);
            let setting = parse_setting(setting);
            let scene = parse_world(world);

            (setting, camera, scene)
        },

        Err(e) => panic!("{:?}", e),
    }
}
