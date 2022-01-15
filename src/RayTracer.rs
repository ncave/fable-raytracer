#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use std::rc::Rc;
use fable_library_rust::*;
pub mod RayTracerDemo {
    use super::*;
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub struct Vector {
        pub X: f64,
        pub Y: f64,
        pub Z: f64,
    }
    pub trait VectorMethods {
        fn op_Multiply(k: &f64, v: &RayTracerDemo::Vector)
        -> RayTracerDemo::Vector;
        fn op_Subtraction(v1: &RayTracerDemo::Vector,
                          v2: &RayTracerDemo::Vector)
        -> RayTracerDemo::Vector;
        fn op_Addition(v1: &RayTracerDemo::Vector, v2: &RayTracerDemo::Vector)
        -> RayTracerDemo::Vector;
        fn Dot(v1: &RayTracerDemo::Vector, v2: &RayTracerDemo::Vector)
        -> f64;
        fn Mag(v: &RayTracerDemo::Vector)
        -> f64;
        fn Norm(v: &RayTracerDemo::Vector)
        -> RayTracerDemo::Vector;
        fn Cross(v1: &RayTracerDemo::Vector, v2: &RayTracerDemo::Vector)
        -> RayTracerDemo::Vector;
        fn Rotate(v: &RayTracerDemo::Vector, e: &RayTracerDemo::Vector,
                  angle: &f64)
        -> RayTracerDemo::Vector;
    }
    impl VectorMethods for Vector {
        fn op_Multiply(k: &f64, v: &RayTracerDemo::Vector)
         -> RayTracerDemo::Vector {
            RayTracerDemo::Vector{X: k.clone() * v.X,
                                  Y: k.clone() * v.Y,
                                  Z: k.clone() * v.Z,}
        }
        fn op_Subtraction(v1: &RayTracerDemo::Vector,
                          v2: &RayTracerDemo::Vector)
         -> RayTracerDemo::Vector {
            RayTracerDemo::Vector{X: v1.X - v2.X,
                                  Y: v1.Y - v2.Y,
                                  Z: v1.Z - v2.Z,}
        }
        fn op_Addition(v1: &RayTracerDemo::Vector, v2: &RayTracerDemo::Vector)
         -> RayTracerDemo::Vector {
            RayTracerDemo::Vector{X: v1.X + v2.X,
                                  Y: v1.Y + v2.Y,
                                  Z: v1.Z + v2.Z,}
        }
        fn Dot(v1: &RayTracerDemo::Vector, v2: &RayTracerDemo::Vector)
         -> f64 {
            v1.X * v2.X + v1.Y * v2.Y + v1.Z * v2.Z
        }
        fn Mag(v: &RayTracerDemo::Vector) -> f64 {
            (v.X * v.X + v.Y * v.Y + v.Z * v.Z).sqrt()
        }
        fn Norm(v: &RayTracerDemo::Vector) -> RayTracerDemo::Vector {
            {
                let mag: f64 = RayTracerDemo::Vector::Mag(v);
                RayTracerDemo::Vector::op_Multiply(&if mag == 0.0f64 {
                                                        f64::INFINITY
                                                    } else { 1.0f64 / mag },
                                                   v)
            }.clone()
        }
        fn Cross(v1: &RayTracerDemo::Vector, v2: &RayTracerDemo::Vector)
         -> RayTracerDemo::Vector {
            RayTracerDemo::Vector{X: v1.Y * v2.Z - v1.Z * v2.Y,
                                  Y: v1.Z * v2.X - v1.X * v2.Z,
                                  Z: v1.X * v2.Y - v1.Y * v2.X,}
        }
        fn Rotate(v: &RayTracerDemo::Vector, e: &RayTracerDemo::Vector,
                  angle: &f64) -> RayTracerDemo::Vector {
            {
                let cosa: f64 = angle.clone().cos();
                let sina: f64 = angle.clone().sin();
                RayTracerDemo::Vector::op_Addition(&RayTracerDemo::Vector::op_Addition(&RayTracerDemo::Vector::op_Multiply(&cosa,
                                                                                                                           v),
                                                                                       &RayTracerDemo::Vector::op_Multiply(&sina,
                                                                                                                           &RayTracerDemo::Vector::Cross(e,
                                                                                                                                                         v))),
                                                   &RayTracerDemo::Vector::op_Multiply(&((1.0f64
                                                                                              -
                                                                                              cosa)
                                                                                             *
                                                                                             RayTracerDemo::Vector::Dot(e,
                                                                                                                        v)),
                                                                                       e))
            }.clone()
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub struct Color {
        pub R: f64,
        pub G: f64,
        pub B: f64,
    }
    pub trait ColorMethods {
        fn Scale(k: &f64, v: &RayTracerDemo::Color)
        -> RayTracerDemo::Color;
        fn op_Addition(v1: &RayTracerDemo::Color, v2: &RayTracerDemo::Color)
        -> RayTracerDemo::Color;
        fn op_Multiply(v1: &RayTracerDemo::Color, v2: &RayTracerDemo::Color)
        -> RayTracerDemo::Color;
        fn White()
        -> RayTracerDemo::Color;
        fn Grey()
        -> RayTracerDemo::Color;
        fn Black()
        -> RayTracerDemo::Color;
        fn Background()
        -> RayTracerDemo::Color;
        fn DefaultColor()
        -> RayTracerDemo::Color;
    }
    impl ColorMethods for Color {
        fn Scale(k: &f64, v: &RayTracerDemo::Color) -> RayTracerDemo::Color {
            RayTracerDemo::Color{R: k.clone() * v.R,
                                 G: k.clone() * v.G,
                                 B: k.clone() * v.B,}
        }
        fn op_Addition(v1: &RayTracerDemo::Color, v2: &RayTracerDemo::Color)
         -> RayTracerDemo::Color {
            RayTracerDemo::Color{R: v1.R + v2.R,
                                 G: v1.G + v2.G,
                                 B: v1.B + v2.B,}
        }
        fn op_Multiply(v1: &RayTracerDemo::Color, v2: &RayTracerDemo::Color)
         -> RayTracerDemo::Color {
            RayTracerDemo::Color{R: v1.R * v2.R,
                                 G: v1.G * v2.G,
                                 B: v1.B * v2.B,}
        }
        fn White() -> RayTracerDemo::Color {
            RayTracerDemo::Color{R: 1.0f64, G: 1.0f64, B: 1.0f64,}
        }
        fn Grey() -> RayTracerDemo::Color {
            RayTracerDemo::Color{R: 0.5f64, G: 0.5f64, B: 0.5f64,}
        }
        fn Black() -> RayTracerDemo::Color {
            RayTracerDemo::Color{R: 0.0f64, G: 0.0f64, B: 0.0f64,}
        }
        fn Background() -> RayTracerDemo::Color {
            RayTracerDemo::Color::Black()
        }
        fn DefaultColor() -> RayTracerDemo::Color {
            RayTracerDemo::Color::Black()
        }
    }
    pub mod RayTracer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Camera {
            pos: RayTracerDemo::Vector,
            lookAt: RayTracerDemo::Vector,
            forward: RayTracerDemo::Vector,
            right: RayTracerDemo::Vector,
            up: RayTracerDemo::Vector,
        }
        impl RayTracerDemo::RayTracer::Camera {
            pub fn new(pos: &RayTracerDemo::Vector,
                       lookAt: &RayTracerDemo::Vector)
             -> Rc<RayTracerDemo::RayTracer::Camera> {
                {
                    let pos_1: RayTracerDemo::Vector;
                    let lookAt_1: RayTracerDemo::Vector;
                    let forward: RayTracerDemo::Vector;
                    let right: RayTracerDemo::Vector;
                    let up: RayTracerDemo::Vector;
                    ();
                    pos_1 = pos.clone();
                    lookAt_1 = lookAt.clone();
                    forward =
                        RayTracerDemo::Vector::Norm(&RayTracerDemo::Vector::op_Subtraction(&lookAt_1,
                                                                                           &pos_1));
                    {
                        let down: RayTracerDemo::Vector =
                            RayTracerDemo::Vector{X: 0.0f64,
                                                  Y: -1.0f64,
                                                  Z: 0.0f64,};
                        right =
                            RayTracerDemo::Vector::op_Multiply(&1.5f64,
                                                               &RayTracerDemo::Vector::Norm(&RayTracerDemo::Vector::Cross(&forward,
                                                                                                                          &down)));
                        up =
                            RayTracerDemo::Vector::op_Multiply(&1.5f64,
                                                               &RayTracerDemo::Vector::Norm(&RayTracerDemo::Vector::Cross(&forward,
                                                                                                                          &right)));
                        ()
                    }
                    Rc::from(RayTracerDemo::RayTracer::Camera{pos:
                                                                  pos_1.clone(),
                                                              lookAt:
                                                                  lookAt_1.clone(),
                                                              forward:
                                                                  forward.clone(),
                                                              right:
                                                                  right.clone(),
                                                              up:
                                                                  up.clone(),})
                }.clone()
            }
        }
        pub trait CameraMethods {
            fn Pos(&self)
            -> RayTracerDemo::Vector;
            fn Forward(&self)
            -> RayTracerDemo::Vector;
            fn Up(&self)
            -> RayTracerDemo::Vector;
            fn Right(&self)
            -> RayTracerDemo::Vector;
            fn Rotate(&self, angle: &f64)
            -> Rc<RayTracerDemo::RayTracer::Camera>;
        }
        impl CameraMethods for Camera {
            fn Pos(&self) -> RayTracerDemo::Vector { self.pos.clone() }
            fn Forward(&self) -> RayTracerDemo::Vector {
                self.forward.clone()
            }
            fn Up(&self) -> RayTracerDemo::Vector { self.up.clone() }
            fn Right(&self) -> RayTracerDemo::Vector { self.right.clone() }
            fn Rotate(&self, angle: &f64)
             -> Rc<RayTracerDemo::RayTracer::Camera> {
                RayTracerDemo::RayTracer::Camera::new(&RayTracerDemo::Vector::Rotate(&RayTracerDemo::Vector::op_Subtraction(&self.pos,
                                                                                                                            &self.lookAt),
                                                                                     &RayTracerDemo::Vector{X:
                                                                                                                0.0f64,
                                                                                                            Y:
                                                                                                                1.0f64,
                                                                                                            Z:
                                                                                                                0.0f64,},
                                                                                     angle),
                                                      &self.lookAt)
            }
        }
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct Ray {
            pub Start: RayTracerDemo::Vector,
            pub Dir: RayTracerDemo::Vector,
        }
        pub trait Surface {
            fn Diffuse(&self, arg0: &RayTracerDemo::Vector)
            -> RayTracerDemo::Color;
            fn Reflect(&self, arg0: &RayTracerDemo::Vector)
            -> f64;
            fn Specular(&self, arg0: &RayTracerDemo::Vector)
            -> RayTracerDemo::Color;
            fn Roughness(&self)
            -> f64;
        }
        #[derive(Clone)]
        pub struct Intersection {
            pub Thing: Rc<dyn RayTracerDemo::RayTracer::SceneObject>,
            pub Ray: RayTracerDemo::RayTracer::Ray,
            pub Dist: f64,
        }
        pub trait SceneObject {
            fn Intersect(&self, arg0: &RayTracerDemo::RayTracer::Ray)
            -> f64;
            fn Normal(&self, arg0: &RayTracerDemo::Vector)
            -> RayTracerDemo::Vector;
            fn Surface(&self)
            -> Rc<dyn RayTracerDemo::RayTracer::Surface>;
        }
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct Light {
            pub Pos: RayTracerDemo::Vector,
            pub Color: RayTracerDemo::Color,
        }
        #[derive(Clone)]
        pub struct Scene {
            pub Things: Rc<MutCell<Vec<Rc<dyn RayTracerDemo::RayTracer::SceneObject>>>>,
            pub Lights: Rc<MutCell<Vec<Rc<RayTracerDemo::RayTracer::Light>>>>,
            pub Camera: Rc<RayTracerDemo::RayTracer::Camera>,
        }
        pub fn maxDepth() -> i32 { 5i32 }
        pub fn NearestIntersection(ray: &RayTracerDemo::RayTracer::Ray,
                                   scene:
                                       &Rc<RayTracerDemo::RayTracer::Scene>)
         -> Option<Rc<RayTracerDemo::RayTracer::Intersection>> {
            {
                let acc:
                        Rc<MutCell<Option<Rc<RayTracerDemo::RayTracer::Intersection>>>> =
                    Rc::from(MutCell::from(None::<Rc<RayTracerDemo::RayTracer::Intersection>>));
                {
                    let arr:
                            Rc<MutCell<Vec<Rc<dyn RayTracerDemo::RayTracer::SceneObject>>>> =
                        scene.Things.clone();
                    for idx in 0i32..=arr.len() as i32 - 1i32 {
                        let x: Rc<dyn RayTracerDemo::RayTracer::SceneObject> =
                            arr[idx].clone();
                        let dist: f64 = x.Intersect(ray);
                        if if acc.get().is_none() {
                               true
                           } else {
                               dist < Option_::getValue(&acc.get()).Dist
                           } {
                            acc.set(Some(Rc::from(RayTracerDemo::RayTracer::Intersection{Thing:
                                                                                             x.clone(),
                                                                                         Ray:
                                                                                             ray.clone(),
                                                                                         Dist:
                                                                                             dist,})));
                        }
                    }
                }
                acc.get().clone()
            }.clone()
        }
        pub fn TestRay(ray: &RayTracerDemo::RayTracer::Ray,
                       scene: &Rc<RayTracerDemo::RayTracer::Scene>)
         -> Option<f64> {
            {
                let matchValue:
                        Option<Rc<RayTracerDemo::RayTracer::Intersection>> =
                    RayTracerDemo::RayTracer::NearestIntersection(ray, scene);
                match &matchValue {
                    Some(matchValue_0_0) =>
                    {
                        let isect:
                                Rc<RayTracerDemo::RayTracer::Intersection> =
                            matchValue_0_0.clone();
                        if isect.Dist == f64::INFINITY {
                            None::<f64>
                        } else { Some(isect.Dist) }
                    }.clone(),
                    _ => None::<f64>,
                }
            }.clone()
        }
        pub fn TraceRay(ray: &RayTracerDemo::RayTracer::Ray,
                        scene: &Rc<RayTracerDemo::RayTracer::Scene>,
                        depth: &i32) -> RayTracerDemo::Color {
            {
                let matchValue:
                        Option<Rc<RayTracerDemo::RayTracer::Intersection>> =
                    RayTracerDemo::RayTracer::NearestIntersection(ray, scene);
                match &matchValue {
                    Some(matchValue_0_0) =>
                    {
                        let isect:
                                Rc<RayTracerDemo::RayTracer::Intersection> =
                            matchValue_0_0.clone();
                        if isect.Dist == f64::INFINITY {
                            RayTracerDemo::Color::Background()
                        } else {
                            RayTracerDemo::RayTracer::Shade(&isect, scene,
                                                            depth)
                        }
                    }.clone(),
                    _ => RayTracerDemo::Color::Background(),
                }
            }.clone()
        }
        pub fn Shade(isect: &Rc<RayTracerDemo::RayTracer::Intersection>,
                     scene: &Rc<RayTracerDemo::RayTracer::Scene>, depth: &i32)
         -> RayTracerDemo::Color {
            {
                let d: RayTracerDemo::Vector = isect.Ray.Dir.clone();
                let pos: RayTracerDemo::Vector =
                    RayTracerDemo::Vector::op_Addition(&RayTracerDemo::Vector::op_Multiply(&isect.Dist,
                                                                                           &d),
                                                       &isect.Ray.Start);
                let normal: RayTracerDemo::Vector = isect.Thing.Normal(&pos);
                let reflectDir: RayTracerDemo::Vector =
                    RayTracerDemo::Vector::op_Subtraction(&d,
                                                          &RayTracerDemo::Vector::op_Multiply(&(2.0f64
                                                                                                    *
                                                                                                    RayTracerDemo::Vector::Dot(&normal,
                                                                                                                               &d)),
                                                                                              &normal));
                RayTracerDemo::Color::op_Addition(&RayTracerDemo::Color::op_Addition(&RayTracerDemo::Color::DefaultColor(),
                                                                                     &RayTracerDemo::RayTracer::GetNaturalColor(&isect.Thing,
                                                                                                                                &pos,
                                                                                                                                &normal,
                                                                                                                                &reflectDir,
                                                                                                                                scene)),
                                                  &if depth.clone() >=
                                                          RayTracerDemo::RayTracer::maxDepth()
                                                      {
                                                       RayTracerDemo::Color::Grey()
                                                   } else {
                                                       RayTracerDemo::RayTracer::GetReflectionColor(&isect.Thing,
                                                                                                    &RayTracerDemo::Vector::op_Addition(&pos,
                                                                                                                                        &RayTracerDemo::Vector::op_Multiply(&0.001f64,
                                                                                                                                                                            &reflectDir)),
                                                                                                    &normal,
                                                                                                    &reflectDir,
                                                                                                    scene,
                                                                                                    depth)
                                                   })
            }.clone()
        }
        pub fn GetReflectionColor(thing:
                                      &Rc<dyn RayTracerDemo::RayTracer::SceneObject>,
                                  pos: &RayTracerDemo::Vector,
                                  normal: &RayTracerDemo::Vector,
                                  rd: &RayTracerDemo::Vector,
                                  scene: &Rc<RayTracerDemo::RayTracer::Scene>,
                                  depth: &i32) -> RayTracerDemo::Color {
            RayTracerDemo::Color::Scale(&thing.Surface().Reflect(pos),
                                        &RayTracerDemo::RayTracer::TraceRay(&RayTracerDemo::RayTracer::Ray{Start:
                                                                                                               pos.clone(),
                                                                                                           Dir:
                                                                                                               rd.clone(),},
                                                                            scene,
                                                                            &(depth.clone()
                                                                                  +
                                                                                  1i32)))
        }
        pub fn GetNaturalColor(thing:
                                   &Rc<dyn RayTracerDemo::RayTracer::SceneObject>,
                               pos: &RayTracerDemo::Vector,
                               normal: &RayTracerDemo::Vector,
                               rd: &RayTracerDemo::Vector,
                               scene: &Rc<RayTracerDemo::RayTracer::Scene>)
         -> RayTracerDemo::Color {
            {
                let color: Rc<MutCell<RayTracerDemo::Color>> =
                    Rc::from(MutCell::from(RayTracerDemo::Color::DefaultColor()));
                {
                    let arr:
                            Rc<MutCell<Vec<Rc<RayTracerDemo::RayTracer::Light>>>> =
                        scene.Lights.clone();
                    for idx in 0i32..=arr.len() as i32 - 1i32 {
                        let light: Rc<RayTracerDemo::RayTracer::Light> =
                            arr[idx].clone();
                        color.set(RayTracerDemo::RayTracer::AddLight(thing,
                                                                     pos,
                                                                     normal,
                                                                     rd,
                                                                     scene,
                                                                     &color.get(),
                                                                     &light))
                    }
                }
                color.get().clone()
            }.clone()
        }
        pub fn AddLight(thing: &Rc<dyn RayTracerDemo::RayTracer::SceneObject>,
                        pos: &RayTracerDemo::Vector,
                        normal: &RayTracerDemo::Vector,
                        rd: &RayTracerDemo::Vector,
                        scene: &Rc<RayTracerDemo::RayTracer::Scene>,
                        color: &RayTracerDemo::Color,
                        light: &Rc<RayTracerDemo::RayTracer::Light>)
         -> RayTracerDemo::Color {
            {
                let ldis: RayTracerDemo::Vector =
                    RayTracerDemo::Vector::op_Subtraction(&light.Pos, pos);
                let livec: RayTracerDemo::Vector =
                    RayTracerDemo::Vector::Norm(&ldis);
                let neatIsect: Option<f64> =
                    RayTracerDemo::RayTracer::TestRay(&RayTracerDemo::RayTracer::Ray{Start:
                                                                                         pos.clone(),
                                                                                     Dir:
                                                                                         livec.clone(),},
                                                      scene);
                if match &neatIsect {
                       Some(neatIsect_0_0) =>
                       !(neatIsect_0_0.clone() >
                             RayTracerDemo::Vector::Mag(&ldis)),
                       _ => false,
                   } {
                    color.clone()
                } else {
                    {
                        let illum: f64 =
                            RayTracerDemo::Vector::Dot(&livec, normal);
                        let lcolor: RayTracerDemo::Color =
                            if illum > 0.0f64 {
                                RayTracerDemo::Color::Scale(&illum,
                                                            &light.Color)
                            } else { RayTracerDemo::Color::DefaultColor() };
                        let specular: f64 =
                            RayTracerDemo::Vector::Dot(&livec,
                                                       &RayTracerDemo::Vector::Norm(rd));
                        let scolor: RayTracerDemo::Color =
                            if specular > 0.0f64 {
                                RayTracerDemo::Color::Scale(&specular.powf(thing.Surface().Roughness()),
                                                            &light.Color)
                            } else { RayTracerDemo::Color::DefaultColor() };
                        RayTracerDemo::Color::op_Addition(&RayTracerDemo::Color::op_Addition(color,
                                                                                             &RayTracerDemo::Color::op_Multiply(&thing.Surface().Diffuse(pos),
                                                                                                                                &lcolor)),
                                                          &RayTracerDemo::Color::op_Multiply(&thing.Surface().Specular(pos),
                                                                                             &scolor))
                    }.clone()
                }
            }.clone()
        }
        pub fn GetPoint(x: &i32, y: &i32, width: &i32, height: &i32,
                        camera: &Rc<RayTracerDemo::RayTracer::Camera>)
         -> RayTracerDemo::Vector {
            RayTracerDemo::Vector::Norm(&RayTracerDemo::Vector::op_Addition(&RayTracerDemo::Vector::op_Addition(&camera.Forward(),
                                                                                                                &RayTracerDemo::Vector::op_Multiply(&((x.clone()
                                                                                                                                                           as
                                                                                                                                                           f64
                                                                                                                                                           -
                                                                                                                                                           width.clone()
                                                                                                                                                               as
                                                                                                                                                               f64
                                                                                                                                                               /
                                                                                                                                                               2.0f64)
                                                                                                                                                          /
                                                                                                                                                          (2.0f64
                                                                                                                                                               *
                                                                                                                                                               width.clone()
                                                                                                                                                                   as
                                                                                                                                                                   f64)),
                                                                                                                                                    &camera.Right())),
                                                                            &RayTracerDemo::Vector::op_Multiply(&(-(y.clone()
                                                                                                                        as
                                                                                                                        f64
                                                                                                                        -
                                                                                                                        height.clone()
                                                                                                                            as
                                                                                                                            f64
                                                                                                                            /
                                                                                                                            2.0f64)
                                                                                                                      /
                                                                                                                      (2.0f64
                                                                                                                           *
                                                                                                                           height.clone()
                                                                                                                               as
                                                                                                                               f64)),
                                                                                                                &camera.Up())))
        }
        pub fn Render(scene: &Rc<RayTracerDemo::RayTracer::Scene>,
                      data: &Rc<MutCell<Vec<u8>>>, x: &i32, y: &i32,
                      width: &i32, height: &i32) {
            fn clamp(v: &f64) -> u8 {
                (v.clone() * 255.0f64).max(0.0f64).min(255.0f64) as u8
            }
            for y_1 in y.clone()..=height.clone() - 1i32 {
                let stride: i32 = y_1 * width.clone();
                for x_1 in x.clone()..=width.clone() - 1i32 {
                    let index: i32 = (x_1 + stride) * 4i32;
                    let dir: RayTracerDemo::Vector =
                        RayTracerDemo::RayTracer::GetPoint(&x_1, &y_1, width,
                                                           height,
                                                           &scene.Camera);
                    let color: RayTracerDemo::Color =
                        RayTracerDemo::RayTracer::TraceRay(&RayTracerDemo::RayTracer::Ray{Start:
                                                                                              scene.Camera.Pos(),
                                                                                          Dir:
                                                                                              dir,},
                                                           scene, &0i32);
                    data.get_mut()[(index + 0i32) as usize] = clamp(&color.R);
                    data.get_mut()[(index + 1i32) as usize] = clamp(&color.G);
                    data.get_mut()[(index + 2i32) as usize] = clamp(&color.B);
                    data.get_mut()[(index + 3i32) as usize] = u8::MAX
                }
            }
        }
        pub fn Rotate(camera: &Rc<RayTracerDemo::RayTracer::Camera>,
                      angle: &f64) -> Rc<RayTracerDemo::RayTracer::Camera> {
            camera.Rotate(angle)
        }
    }
    pub mod SceneObjects {
        use super::*;
        #[derive(Clone)]
        pub struct Sphere {
            surface: Rc<dyn RayTracerDemo::RayTracer::Surface>,
            radius: f64,
            center: RayTracerDemo::Vector,
        }
        impl RayTracerDemo::SceneObjects::Sphere {
            pub fn new(center: &RayTracerDemo::Vector, radius: &f64,
                       surface: &Rc<dyn RayTracerDemo::RayTracer::Surface>)
             -> Rc<RayTracerDemo::SceneObjects::Sphere> {
                {
                    let surface_1: Rc<dyn RayTracerDemo::RayTracer::Surface>;
                    let radius_1: f64;
                    let center_1: RayTracerDemo::Vector;
                    ();
                    center_1 = center.clone();
                    radius_1 = radius.clone();
                    surface_1 = surface.clone();
                    ();
                    Rc::from(RayTracerDemo::SceneObjects::Sphere{surface:
                                                                     surface_1.clone(),
                                                                 radius:
                                                                     radius_1,
                                                                 center:
                                                                     center_1.clone(),})
                }.clone()
            }
        }
        impl RayTracerDemo::RayTracer::SceneObject for Sphere {
            fn Surface(&self) -> Rc<dyn RayTracerDemo::RayTracer::Surface> {
                self.surface.clone()
            }
            fn Normal(&self, pos: &RayTracerDemo::Vector)
             -> RayTracerDemo::Vector {
                RayTracerDemo::Vector::Norm(&RayTracerDemo::Vector::op_Subtraction(pos,
                                                                                   &self.center))
            }
            fn Intersect(&self, ray: &RayTracerDemo::RayTracer::Ray) -> f64 {
                let eo: RayTracerDemo::Vector =
                    RayTracerDemo::Vector::op_Subtraction(&self.center,
                                                          &ray.Start);
                let v: f64 = RayTracerDemo::Vector::Dot(&eo, &ray.Dir);
                if v < 0.0f64 {
                    f64::INFINITY
                } else {
                    let disc: f64 =
                        self.radius * self.radius -
                            (RayTracerDemo::Vector::Dot(&eo, &eo) - v * v);
                    if disc < 0.0f64 {
                        f64::INFINITY
                    } else { v - disc.sqrt() }
                }
            }
        }
        #[derive(Clone)]
        pub struct Plane {
            surface: Rc<dyn RayTracerDemo::RayTracer::Surface>,
            offset: f64,
            normal: RayTracerDemo::Vector,
        }
        impl RayTracerDemo::SceneObjects::Plane {
            pub fn new(normal: &RayTracerDemo::Vector, offset: &f64,
                       surface: &Rc<dyn RayTracerDemo::RayTracer::Surface>)
             -> Rc<RayTracerDemo::SceneObjects::Plane> {
                {
                    let surface_1: Rc<dyn RayTracerDemo::RayTracer::Surface>;
                    let offset_1: f64;
                    let normal_1: RayTracerDemo::Vector;
                    ();
                    normal_1 = normal.clone();
                    offset_1 = offset.clone();
                    surface_1 = surface.clone();
                    ();
                    Rc::from(RayTracerDemo::SceneObjects::Plane{surface:
                                                                    surface_1.clone(),
                                                                offset:
                                                                    offset_1,
                                                                normal:
                                                                    normal_1.clone(),})
                }.clone()
            }
        }
        impl RayTracerDemo::RayTracer::SceneObject for Plane {
            fn Surface(&self) -> Rc<dyn RayTracerDemo::RayTracer::Surface> {
                self.surface.clone()
            }
            fn Normal(&self, pos: &RayTracerDemo::Vector)
             -> RayTracerDemo::Vector {
                self.normal.clone()
            }
            fn Intersect(&self, ray: &RayTracerDemo::RayTracer::Ray) -> f64 {
                let denom: f64 =
                    RayTracerDemo::Vector::Dot(&self.normal, &ray.Dir);
                if denom > 0.0f64 {
                    f64::INFINITY
                } else {
                    (RayTracerDemo::Vector::Dot(&self.normal, &ray.Start) +
                         self.offset) / -denom
                }
            }
        }
    }
    pub mod Surfaces {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Shiny {
        }
        impl RayTracerDemo::Surfaces::Shiny {
            pub fn new() -> Rc<RayTracerDemo::Surfaces::Shiny> {
                { (); (); Rc::from(RayTracerDemo::Surfaces::Shiny{}) }.clone()
            }
        }
        impl RayTracerDemo::RayTracer::Surface for Shiny {
            fn Diffuse(&self, pos: &RayTracerDemo::Vector)
             -> RayTracerDemo::Color {
                RayTracerDemo::Color::White()
            }
            fn Specular(&self, pos: &RayTracerDemo::Vector)
             -> RayTracerDemo::Color {
                RayTracerDemo::Color::Grey()
            }
            fn Reflect(&self, pos: &RayTracerDemo::Vector) -> f64 { 0.7f64 }
            fn Roughness(&self) -> f64 { 250.0f64 }
        }
        #[derive(Clone, Debug)]
        pub struct Checkerboard {
        }
        impl RayTracerDemo::Surfaces::Checkerboard {
            pub fn new() -> Rc<RayTracerDemo::Surfaces::Checkerboard> {
                {
                    ();
                    ();
                    Rc::from(RayTracerDemo::Surfaces::Checkerboard{})
                }.clone()
            }
        }
        impl RayTracerDemo::RayTracer::Surface for Checkerboard {
            fn Diffuse(&self, pos: &RayTracerDemo::Vector)
             -> RayTracerDemo::Color {
                if (pos.Z.floor() + pos.X.floor()) as i32 % 2i32 != 0i32 {
                    RayTracerDemo::Color::White()
                } else { RayTracerDemo::Color::Black() }
            }
            fn Specular(&self, pos: &RayTracerDemo::Vector)
             -> RayTracerDemo::Color {
                RayTracerDemo::Color::White()
            }
            fn Reflect(&self, pos: &RayTracerDemo::Vector) -> f64 {
                if (pos.Z.floor() + pos.X.floor()) as i32 % 2i32 != 0i32 {
                    0.1f64
                } else { 0.7f64 }
            }
            fn Roughness(&self) -> f64 { 150.0f64 }
        }
    }
    pub mod Scenes {
        use super::*;
        pub fn TwoSpheresOnACheckerboard()
         -> Rc<RayTracerDemo::RayTracer::Scene> {
            Rc::from(RayTracerDemo::RayTracer::Scene{Things:
                                                         Native::arrayFrom(&[RayTracerDemo::SceneObjects::Plane::new(&RayTracerDemo::Vector{X:
                                                                                                                                                0.0f64,
                                                                                                                                            Y:
                                                                                                                                                1.0f64,
                                                                                                                                            Z:
                                                                                                                                                0.0f64,},
                                                                                                                     &0.0f64,
                                                                                                                     &(RayTracerDemo::Surfaces::Checkerboard::new().clone()
                                                                                                                           as
                                                                                                                           Rc<dyn RayTracerDemo::RayTracer::Surface>)).clone()
                                                                                 as
                                                                                 Rc<dyn RayTracerDemo::RayTracer::SceneObject>,
                                                                             RayTracerDemo::SceneObjects::Sphere::new(&RayTracerDemo::Vector{X:
                                                                                                                                                 0.0f64,
                                                                                                                                             Y:
                                                                                                                                                 1.0f64,
                                                                                                                                             Z:
                                                                                                                                                 -0.25f64,},
                                                                                                                      &1.0f64,
                                                                                                                      &(RayTracerDemo::Surfaces::Shiny::new().clone()
                                                                                                                            as
                                                                                                                            Rc<dyn RayTracerDemo::RayTracer::Surface>)).clone()
                                                                                 as
                                                                                 Rc<dyn RayTracerDemo::RayTracer::SceneObject>,
                                                                             RayTracerDemo::SceneObjects::Sphere::new(&RayTracerDemo::Vector{X:
                                                                                                                                                 -1.0f64,
                                                                                                                                             Y:
                                                                                                                                                 0.5f64,
                                                                                                                                             Z:
                                                                                                                                                 1.5f64,},
                                                                                                                      &0.5f64,
                                                                                                                      &(RayTracerDemo::Surfaces::Shiny::new().clone()
                                                                                                                            as
                                                                                                                            Rc<dyn RayTracerDemo::RayTracer::Surface>)).clone()
                                                                                 as
                                                                                 Rc<dyn RayTracerDemo::RayTracer::SceneObject>]),
                                                     Lights:
                                                         Native::arrayFrom(&[Rc::from(RayTracerDemo::RayTracer::Light{Pos:
                                                                                                                          RayTracerDemo::Vector{X:
                                                                                                                                                    -2.0f64,
                                                                                                                                                Y:
                                                                                                                                                    2.5f64,
                                                                                                                                                Z:
                                                                                                                                                    0.0f64,},
                                                                                                                      Color:
                                                                                                                          RayTracerDemo::Color{R:
                                                                                                                                                   0.49f64,
                                                                                                                                               G:
                                                                                                                                                   0.07f64,
                                                                                                                                               B:
                                                                                                                                                   0.07f64,},}),
                                                                             Rc::from(RayTracerDemo::RayTracer::Light{Pos:
                                                                                                                          RayTracerDemo::Vector{X:
                                                                                                                                                    1.5f64,
                                                                                                                                                Y:
                                                                                                                                                    2.5f64,
                                                                                                                                                Z:
                                                                                                                                                    1.5f64,},
                                                                                                                      Color:
                                                                                                                          RayTracerDemo::Color{R:
                                                                                                                                                   0.07f64,
                                                                                                                                               G:
                                                                                                                                                   0.07f64,
                                                                                                                                               B:
                                                                                                                                                   0.49f64,},}),
                                                                             Rc::from(RayTracerDemo::RayTracer::Light{Pos:
                                                                                                                          RayTracerDemo::Vector{X:
                                                                                                                                                    1.5f64,
                                                                                                                                                Y:
                                                                                                                                                    2.5f64,
                                                                                                                                                Z:
                                                                                                                                                    -1.5f64,},
                                                                                                                      Color:
                                                                                                                          RayTracerDemo::Color{R:
                                                                                                                                                   0.07f64,
                                                                                                                                               G:
                                                                                                                                                   0.49f64,
                                                                                                                                               B:
                                                                                                                                                   0.071f64,},}),
                                                                             Rc::from(RayTracerDemo::RayTracer::Light{Pos:
                                                                                                                          RayTracerDemo::Vector{X:
                                                                                                                                                    0.0f64,
                                                                                                                                                Y:
                                                                                                                                                    3.5f64,
                                                                                                                                                Z:
                                                                                                                                                    0.0f64,},
                                                                                                                      Color:
                                                                                                                          RayTracerDemo::Color{R:
                                                                                                                                                   0.21f64,
                                                                                                                                               G:
                                                                                                                                                   0.21f64,
                                                                                                                                               B:
                                                                                                                                                   0.35f64,},})]),
                                                     Camera:
                                                         RayTracerDemo::RayTracer::Camera::new(&RayTracerDemo::Vector{X:
                                                                                                                          3.0f64,
                                                                                                                      Y:
                                                                                                                          2.0f64,
                                                                                                                      Z:
                                                                                                                          4.0f64,},
                                                                                               &RayTracerDemo::Vector{X:
                                                                                                                          -1.0f64,
                                                                                                                      Y:
                                                                                                                          0.5f64,
                                                                                                                      Z:
                                                                                                                          0.0f64,}),})
        }
    }
    pub fn renderScene(data: &Rc<MutCell<Vec<u8>>>, x: &i32, y: &i32, w: &i32,
                       h: &i32, angle: &f64) {
        let scene: Rc<RayTracerDemo::RayTracer::Scene> =
            RayTracerDemo::Scenes::TwoSpheresOnACheckerboard();
        RayTracerDemo::RayTracer::Render(&Rc::from(RayTracerDemo::RayTracer::Scene{Things:
                                                                                       scene.Things.clone(),
                                                                                   Lights:
                                                                                       scene.Lights.clone(),
                                                                                   Camera:
                                                                                       RayTracerDemo::RayTracer::Rotate(&scene.Camera,
                                                                                                                        angle),}),
                                         data, x, y, w, h)
    }
}
