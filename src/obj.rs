/*

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::collidable::{Collidable, Collision, Ray};



pub struct Model {
    pub verts : Vec<Vector>,
    pub faces : Vec<u32>, //point to three verts of Vec verts
    //TODO text coords and indecies
    pub origin : Vector,
    pub scale : Vector,
}

impl Collidable for Model {
    fn intersect(&self, ray : &Ray) -> Collision {
        //loop though all faces
        let mut i = 0;
        while i < self.faces.len() {
            //get 3 vertecies
            let mut v0 : Vector = self.verts[self.faces[i + 0] as usize];
            let mut v1 : Vector = self.verts[self.faces[i + 1] as usize];
            let mut v2 : Vector = self.verts[self.faces[i + 2] as usize];

            v0 = Vector::vec_mul(&v0, &self.scale) + self.origin;
            v1 = Vector::vec_mul(&v1, &self.scale) + self.origin;
            v2 = Vector::vec_mul(&v2, &self.scale) + self.origin;

            let edge1 = v0 - v1;
            let edge2 = v0 - v2;
            let edge3 = v1 - v2;
            let normal : Vector = Vector::cross(&edge1, &edge2).normalize(); //Get normal of face
            

            let D : f32 = normal * v0;

            let denominator = normal * ray.d;

            if denominator == 0.0 {
                //ray is parallel to plane -> won't intersect
                //continue test with different triangles
                i += 3;
                continue;
            }

            let t : f32 = - ((normal * ray.o) + D) / denominator;

            if t <= 0.0 {
                //intersection behind origion, not visible
                i += 3;
                continue;
            }


            let poi : Vector = ray.o + ray.d * t;


            //inside out test
            let C0 : Vector = poi - v0;
            let C1 : Vector = poi - v1;
            let C2 : Vector = poi - v2;

            if  normal * Vector::cross(&edge1, &C0) > 0.0 &&
                normal * Vector::cross(&edge2, &C1) > 0.0 &&
                normal * Vector::cross(&edge3, &C2) > 0.0  {
                    //inside triangle
                    println!("{}", poi.to_string());
                    return Collision{t, poi, normal};
            }
            
            //outside 
            //add 3 to i -> next face
            i += 3;
        }
        Collision{t : -1.0, poi: Vector::ZERO, normal : Vector::ZERO}


    }
}




impl Model {

    //Static function to load model
    pub fn load_from_file(filename : &str) -> Model {
        println!("Loading model from file {}", filename);
        let mut verts : Vec<Vector> = Vec::new();
        let mut verts_index : Vec<u32> = Vec::new();
        let mut normals : Vec<Vector> = Vec::new();
        let origin = Vector::new(0.0, 0.0, 0.0);
        let scale = Vector::new(100.0, 100.0, 100.0);

        let f = File::open(filename).expect("File not found!");
        let mut file = BufReader::new(&f);

        for line in file.lines() {
            let l = line.unwrap();

            let split: Vec<&str> = l.split(" ").collect();
            if split[0] == "v" {
                //Vertex
                let x : f32 = split[1].parse().unwrap();
                let y : f32 = split[2].parse().unwrap();
                let z : f32 = split[3].parse().unwrap();
                let mut v = Vector::new(x, y, z);
                println!("Adding Vertex {}", v.to_string());
                verts.push(v);
            }

            else if split[0] == "f"{
                //Face definition
                let mut viVec : Vec<u32> = Vec::new();
                //let mut tiVec : Vec<u32> = Vec::new();

                for i in 1..4 {

                    let d: Vec<&str> = split[i].split("/").collect();

                    //first get Pointer to Vertecies
                    let vi : u32 = d[0].parse().unwrap();
                    //let ti : u32 = d[1].parse().unwrap();
                    viVec.push(vi - 1);
                    //tiVec.push(ti - 1);

                }
                //add vert pointers to vec
                verts_index.append(&mut viVec);

            }
        }

        Model{verts, faces : verts_index, origin, scale}
    }

}
*/