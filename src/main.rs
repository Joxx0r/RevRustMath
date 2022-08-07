

use libm::*;

#[derive(Debug, PartialEq)]
struct rev_v2
{
    x:f32,
    y:f32
}

impl rev_v2 {

    pub fn zero() -> Self{  
        Self::new_1(0.0)
    }
    pub fn new() -> Self {  
        Self{..Default::default()}
    }
    pub fn new_0() -> Self {  
        Self{..Default::default()}
    }
    pub fn new_1(in_s:f32) -> Self  {   
        Self{x:in_s, y:in_s}
    } 
    
    pub fn length(&self) -> f32
    {
        return sqrtf(self.length2());
    }

    pub fn length2(&self) -> f32
    {
        return self.x * self.x + self.y * self.y;
    }
}

impl Default for rev_v2 {
    fn default() -> Self {
        Self { 
            x:0.0,
            y:0.0,
        }
    }
}

#[derive(Debug, PartialEq)]
struct rev_v3
{
    x:f32,
    y:f32,
    z:f32,
}

impl rev_v3 {
    pub fn zero() -> Self{  
        Self::new_1(0.0)
    }
    pub fn new() -> Self {  
        Self{..Default::default()}
    }
    pub fn new_0() -> Self {  
        Self{..Default::default()}
    }
    pub fn new_1(in_s:f32) -> Self  {   
        Self{x:in_s, y:in_s, z:in_s}
    } 
    pub fn new_3(in_x:f32, in_y:f32, in_z:f32) -> Self {   
        Self{x:in_x, y:in_y, z:in_z}
    }
    
    pub fn length(&self) -> f32
    {
        return sqrtf(self.length2());
    }

    pub fn length2(&self) -> f32
    {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
}

impl Default for rev_v3 {
    fn default() -> Self {
        Self {
            x:0.0,
            y:0.0,
            z:0.0,
        }
    }
}


#[derive(Debug, PartialEq)]
struct rev_v4
{
    x:f32,
    y:f32,
    z:f32,
    w:f32,
}


impl rev_v4 {

    pub fn length(&self) -> f32
    {
        return sqrtf(self.length2());
    }

    pub fn length2(&self) -> f32
    {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
    }

    pub fn zero() -> Self{  
        Self::new_1(0.0)
    }

    pub fn new() -> Self {  
        Self{..Default::default()}
    }
    pub fn new_0() -> Self {  
        Self{..Default::default()}
    }
    pub fn new_1(in_s:f32) -> Self  {   
        Self{x:in_s, y:in_s, z:in_s, w:0.0}
    } 
    pub fn new_3(in_x:f32, in_y:f32, in_z:f32) -> Self {   
        Self{x:in_x, y:in_y, z:in_z, ..Default::default()}
    }
    pub fn new_4(in_x:f32, in_y:f32, in_z:f32, in_w:f32) -> Self {   
        Self{x:in_x, y:in_y, z:in_z, w:in_w}
    }
}

impl Default for rev_v4 {
    fn default() -> Self {
        rev_v4 { 
            x:0.0,
            y:0.0,
            z:0.0,
            w:1.0,
        }
    }
}

fn main() {
    let v = rev_v4::default();
    let v_length = v.length();
    println!("Hello, world!");
}
