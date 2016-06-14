use std::ops::*;

///A type to hold 2d Point data
#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Point2D<T:Clone + Copy + Add + Sub + Mul + Div>{
    pub x : T,
    pub y : T,
}

impl<T:Clone + Copy + Add + Sub + Mul + Div> Point2D<T>{
    ///creates a new Point2D instance
    pub fn new(x:T,y:T)->Point2D<T>{
        Point2D{
            x : x,
            y : y,
        }
    }
}

impl<T> Add for Point2D<T> where T:Clone + Copy + Add<Output = T> + Sub + Mul + Div {
    type Output = Point2D<T>;
    fn add(self,other:Point2D<T>)->Point2D<T>{
        Point2D::new(self.x+other.x,self.y+other.y)
    }
}

impl<T> AddAssign for Point2D<T> where T:Clone + Copy + Add<Output = T> + Sub + Mul + Div {
    fn add_assign(&mut self,other:Point2D<T>){
        self.x = self.x+other.x;
        self.y = self.y+other.y;
    }
}

impl<T> Sub for Point2D<T> where T:Clone + Copy + Add + Sub<Output = T> + Mul + Div {
    type Output = Point2D<T>;
    fn sub(self,other:Point2D<T>)->Point2D<T>{
        Point2D::new(self.x-other.x,self.y-other.y)
    }
}

impl<T> SubAssign for Point2D<T> where T:Clone + Copy + Add + Sub<Output = T> + Mul + Div {
    fn sub_assign(&mut self,other:Point2D<T>){
        self.x = self.x-other.x;
        self.y = self.y-other.y;
    }
}

impl<T> Mul for Point2D<T> where T:Clone + Copy + Add<Output = T> + Sub + Mul<Output = T> + Div {
    type Output = T;
    ///dot product
    fn mul(self,other:Point2D<T>)->T{
        self.x*other.x+self.y*other.y
    }
}

impl Mul<f32> for Point2D<f32> {
    type Output = Point2D<f32>;
    ///scalar multiplication
    fn mul(self,other:f32)->Point2D<f32>{
        Point2D::new(self.x*other,self.y*other)
    }
}

impl Mul<f64> for Point2D<f64> {
    type Output = Point2D<f64>;
    ///scalar multiplication
    fn mul(self,other:f64)->Point2D<f64>{
        Point2D::new(self.x*other,self.y*other)
    }
}

impl Mul<i32> for Point2D<i32> {
    type Output = Point2D<i32>;
    ///scalar multiplication
    fn mul(self,other:i32)->Point2D<i32>{
        Point2D::new(self.x*other,self.y*other)
    }
}

impl Mul<i64> for Point2D<i64> {
    type Output = Point2D<i64>;
    ///scalar multiplication
    fn mul(self,other:i64)->Point2D<i64>{
        Point2D::new(self.x*other,self.y*other)
    }
}

impl MulAssign<f32> for Point2D<f32> {
    ///scalar multiplication
    fn mul_assign(&mut self,other:f32){
        self.x = self.x*other;
        self.y = self.y*other;
    }
}

impl MulAssign<f64> for Point2D<f64> {
    ///scalar multiplication
    fn mul_assign(&mut self,other:f64){
        self.x = self.x*other;
        self.y = self.y*other;
    }
}

impl MulAssign<i32> for Point2D<i32> {
    ///scalar multiplication
    fn mul_assign(&mut self,other:i32){
        self.x = self.x*other;
        self.y = self.y*other;
    }
}

impl MulAssign<i64> for Point2D<i64> {
    ///scalar multiplication
    fn mul_assign(&mut self,other:i64){
        self.x = self.x*other;
        self.y = self.y*other;
    }
}

impl Div<f32> for Point2D<f32> {
    type Output = Point2D<f32>;
    ///scalar division
    fn div(self,other:f32)->Point2D<f32>{
        Point2D::new(self.x/other,self.y/other)
    }
}

impl Div<f64> for Point2D<f64> {
    type Output = Point2D<f64>;
    ///scalar division
    fn div(self,other:f64)->Point2D<f64>{
        Point2D::new(self.x/other,self.y/other)
    }
}

impl Div<i32> for Point2D<i32> {
    type Output = Point2D<i32>;
    ///scalar division
    fn div(self,other:i32)->Point2D<i32>{
        Point2D::new(self.x/other,self.y/other)
    }
}

impl Div<i64> for Point2D<i64> {
    type Output = Point2D<i64>;
    ///scalar division
    fn div(self,other:i64)->Point2D<i64>{
        Point2D::new(self.x/other,self.y/other)
    }
}

impl DivAssign<f32> for Point2D<f32> {
    ///scalar division
    fn div_assign(&mut self,other:f32){
        self.x = self.x/other;
        self.y = self.y/other;
    }
}

impl DivAssign<f64> for Point2D<f64> {
    ///scalar division
    fn div_assign(&mut self,other:f64){
        self.x = self.x/other;
        self.y = self.y/other;
    }
}

impl DivAssign<i32> for Point2D<i32> {
    ///scalar division
    fn div_assign(&mut self,other:i32){
        self.x = self.x/other;
        self.y = self.y/other;
    }
}

impl DivAssign<i64> for Point2D<i64> {
    ///scalar division
    fn div_assign(&mut self,other:i64){
        self.x = self.x/other;
        self.y = self.y/other;
    }
}

#[cfg(test)]
mod tests{
    use point2d::*;
    #[test]
    fn point_2d_adding()
    {
        let expected:Point2D<i32> = Point2D::new(10,-2);
        let p1:Point2D<i32> = Point2D::new(2,4);
        let p2:Point2D<i32> = Point2D::new(8,-6);
        let mut p3:Point2D<i32> = Point2D::new(-10,10);
        let p4:Point2D<i32> = Point2D::new(20,-12);
        p3+=p4;
        assert_eq!(expected,p1+p2);
        assert_eq!(expected,p3);
    }

    #[test]
    fn point_2d_subtraction()
    {
        let expected:Point2D<i32> =  Point2D::new(50,10);
        let p1:Point2D<i32> = Point2D::new(125,-5);
        let p2:Point2D<i32> = Point2D::new(75,-15);
        let mut p3:Point2D<i32> = Point2D::new(-50,20);
        let p4:Point2D<i32> = Point2D::new(-100,10);
        p3-=p4;
        assert_eq!(expected,p1-p2);
        assert_eq!(expected,p3);
    }

    #[test]
    fn point_2d_scalar_multiply()
    {
        let expectedf32:Point2D<f32> = Point2D::new(10.0,2.5);
        let p1f32:Point2D<f32> = Point2D::new(5.0,1.25);
        assert_eq!(expectedf32,p1f32*2.0);
        let mut p2f32:Point2D<f32> = Point2D::new(2.5,0.625);
        p2f32*=4.0;
        assert_eq!(expectedf32,p2f32);

        let expectedf64:Point2D<f64> = Point2D::new(20.0,5.0);
        let p1f64:Point2D<f64> = Point2D::new(2.0,0.5);
        assert_eq!(expectedf64,p1f64*10.0);
        let mut p2f64:Point2D<f64> = Point2D::new(4.0,1.0);
        p2f64*=5.0;
        assert_eq!(expectedf64,p2f64);

        let expectedi32:Point2D<i32> = Point2D::new(60,120);
        let p1i32:Point2D<i32> = Point2D::new(12,24);
        assert_eq!(expectedi32,p1i32*5);
        let mut p2i32:Point2D<i32> = Point2D::new(10,20);
        p2i32*=6;
        assert_eq!(expectedi32,p2i32);

        let expectedi64:Point2D<i64> = Point2D::new(100,200);
        let p1i64:Point2D<i64> = Point2D::new(10,20);
        assert_eq!(expectedi64,p1i64*10);
        let mut p2i64:Point2D<i64> = Point2D::new(50,100);
        p2i64*=2;
        assert_eq!(expectedi64,p2i64);
    }

    #[test]
    fn point_2d_dot_multiply()
    {
        let expected:i32 = 46;
        let p1:Point2D<i32> = Point2D::new(10,2);
        let p2:Point2D<i32> = Point2D::new(4,3);
        assert_eq!(expected,p1*p2);
    }

    #[test]
    fn point_2d_scalar_divide()
    {
        let expectedf32:Point2D<f32> = Point2D::new(2.5,1.0);
        let p1f32:Point2D<f32> = Point2D::new(5.0,2.0);
        assert_eq!(expectedf32,p1f32/2.0);
        let mut p2f32:Point2D<f32> = Point2D::new(10.0,4.0);
        p2f32/=4.0;
        assert_eq!(expectedf32,p2f32);

        let expectedf64:Point2D<f64> =  Point2D::new(1.25,0.25);
        let p1f64:Point2D<f64> = Point2D::new(10.0,2.0);
        assert_eq!(expectedf64,p1f64/8.0);
        let mut p2f64:Point2D<f64> = Point2D::new(5.0,1.0);
        p2f64/=4.0;
        assert_eq!(expectedf64,p2f64);

        let expectedi32:Point2D<i32> = Point2D::new(6,12);
        let p1i32:Point2D<i32> = Point2D::new(60,120);
        assert_eq!(expectedi32,p1i32/10);
        let mut p2i32:Point2D<i32> = Point2D::new(12,24);
        p2i32/=2;
        assert_eq!(expectedi32,p2i32);

        let expectedi64:Point2D<i64> = Point2D::new(12,5);
        let p1i64:Point2D<i64> = Point2D::new(24,10);
        assert_eq!(expectedi64,p1i64/2);
        let mut p2i64:Point2D<i64> = Point2D::new(120,50);
        p2i64/=10;
        assert_eq!(expectedi64,p2i64);
    }
}