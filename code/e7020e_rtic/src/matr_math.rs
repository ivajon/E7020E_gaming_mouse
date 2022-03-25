//! Defines some basic math functions



/// Defines a 3 element matrix
#[derive(Copy, Clone)]
pub struct vec_3 {
    pub data : [f32;3],
}
/// Defines a 3x3 matrix
#[derive(Copy, Clone)]
pub struct matr_3 {
    data : [vec_3;3],
    pub enabled : bool
}

pub fn remap(val :u8)->f32{
    (f32::from(val))/255.0
}
impl vec_3{
    pub fn from_color_values(r : u8, g : u8, b : u8)->vec_3{
        vec_3 { data: [remap(r),remap(g),remap(b)] }
    }
    pub fn new(data : [f32;3])-> vec_3{
        vec_3 { data: data }
    }
    /// Limits the values of the vector to be between 0 and 1
    pub fn cap(&mut self){
        for i in 0..2{
            self.data[i] = self.data[i].min(1.0);
            self.data[i] = self.data[i].max(0.0);
        }
    }
    pub fn dot_prod(&mut self,other : vec_3)->vec_3{
        let mut ret = vec_3 :: new([0.0,0.0,0.0]);
        for i in 0..2{
            ret.data[i] = self.data[i]*other.data[i];
        }
        ret.cap();
        ret
    }
    /// Adds 2 vectors in r3
    pub fn add(&mut self,other : vec_3)->vec_3{
        let mut ret : vec_3 = vec_3::new( self.data );
        for i in 0..2{
            ret.data[i]+=other.data[i];
        }
        ret.cap();
        ret
    }

    // ================ Scalar operations ================
    /// Scales a vector by a float
    pub fn scale(&mut self,scalar : f32){
        for i in 0..2{
            self.data[i]*=scalar;
        }
        self.cap()
    }
}   

impl matr_3{
    /// initiates a new a new 3x3 matrix
    pub fn new(data : [[f32;3];3],enabled : bool)->matr_3{
        matr_3 { 
            data: [vec_3::new(data[0]),vec_3::new(data[1]),vec_3::new(data[2])],
            enabled : enabled
        }
    }

    /// multiplies 2 matricies
    /// standard ugly multiplication
    pub fn mul(&mut self,other : matr_3)->matr_3{
        let mut ret : matr_3 = matr_3::new([[0.0,0.0,0.0],[0.0,0.0,0.0],[0.0,0.0,0.0]],self.enabled);
        for i in 0..3{
            for j in 0..3{
                for k in 0..3{
                    ret.data[i].data[j] += self.data[i].data[k]*other.data[k].data[j];
                }
            }
        }
        ret
    }
    /// Multiplies a vector with a matrix
    /// Simple dumb multiplication
    pub fn mul_vec(&mut self,other : vec_3)->vec_3{
        let mut ret : vec_3 = vec_3::new([0.0,0.0,0.0]);
        for i in 0..3{
            for j in 0..3{
                ret.data[i] += self.data[i].data[j]*other.data[j];
            }
        }
        ret
    }
    

}