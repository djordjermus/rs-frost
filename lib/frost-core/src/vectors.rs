#[derive(Clone)]
pub struct Vector2<T> { pub x: T, pub y: T }
impl<T> Vector2<T>
{
    pub fn new(x: T, y: T) -> Self
    {
        return Self { x: x, y: y };
    }
    pub fn scale(self, scalar: T) -> Self where T : std::ops::Mul<T, Output = T>, T : Copy
    {
        return Self { x: self.x * scalar, y: self.y * scalar };
    }
}

impl<T> Copy for Vector2<T> where T : std::marker::Copy {}

impl<T> std::fmt::Display for Vector2<T> where T : std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "[x: {}, y: {}]", self.x, self.y);
    }
}

impl<T, U> std::ops::Add<Vector2<U>> for Vector2<T> where T : std::ops::Add<U, Output = T>
{
    type Output = Vector2<T>;
    fn add(self, rhs: Vector2<U>) -> Self::Output
    {
        return Self { x: self.x + rhs.x, y: self.y + rhs.y };
    }
}

impl<T, U> std::ops::Sub<Vector2<U>> for Vector2<T> where T : std::ops::Sub<U, Output = T>
{
    type Output = Vector2<T>;
    fn sub(self, rhs: Vector2<U>) -> Self::Output
    {
        return Self { x: self.x - rhs.x, y: self.y - rhs.y };
    }
}

impl<T, U> std::ops::Mul<Vector2<U>> for Vector2<T> where T : std::ops::Mul<U, Output = T>
{
    type Output = Vector2<T>;
    fn mul(self, rhs: Vector2<U>) -> Self::Output
    {
        return Self { x: self.x * rhs.x, y: self.y * rhs.y };
    }
}

impl<T, U> std::ops::Div<Vector2<U>> for Vector2<T> where T : std::ops::Div<U, Output = T>
{
    type Output = Vector2<T>;
    fn div(self, rhs: Vector2<U>) -> Self::Output
    {
        return Self { x: self.x / rhs.x, y: self.y / rhs.y };
    }
}

#[derive(Clone)]
pub struct Vector3<T> { pub x: T, pub y: T, pub z: T }
impl<T> Vector3<T>
{
    pub fn new(x: T, y: T, z: T) -> Self
    {
        return Self { x: x, y: y, z: z };
    }
    pub fn scale(self, scalar: T) -> Self where T : std::ops::Mul<T, Output = T>, T : Copy
    {
        return Self { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar };
    }
}

impl<T> Copy for Vector3<T> where T : std::marker::Copy {}

impl<T> std::fmt::Display for Vector3<T> where T : std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "[x: {}, y: {}, z: {}]", self.x, self.y, self.z);
    }
}

impl<T, U> std::ops::Add<Vector3<U>> for Vector3<T> where T : std::ops::Add<U, Output = T>
{
    type Output = Vector3<T>;
    fn add(self, rhs: Vector3<U>) -> Self::Output
    {
        return Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z };
    }
}

impl<T, U> std::ops::Sub<Vector3<U>> for Vector3<T> where T : std::ops::Sub<U, Output = T>
{
    type Output = Vector3<T>;
    fn sub(self, rhs: Vector3<U>) -> Self::Output
    {
        return Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z };
    }
}

impl<T, U> std::ops::Mul<Vector3<U>> for Vector3<T> where T : std::ops::Mul<U, Output = T>
{
    type Output = Vector3<T>;
    fn mul(self, rhs: Vector3<U>) -> Self::Output
    {
        return Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z };
    }
}

impl<T, U> std::ops::Div<Vector3<U>> for Vector3<T> where T : std::ops::Div<U, Output = T>
{
    type Output = Vector3<T>;
    fn div(self, rhs: Vector3<U>) -> Self::Output
    {
        return Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z };
    }
}


#[derive(Clone)]
pub struct Vector4<T> { pub x: T, pub y: T, pub z: T, pub w: T }
impl<T> Vector4<T>
{
    pub fn new(x: T, y: T, z: T, w: T) -> Self
    {
        return Self { x: x, y: y, z: z, w: w };
    }
    pub fn scale(self, scalar: T) -> Self where T : std::ops::Mul<T, Output = T>, T : Copy
    {
        return Self { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar, w: self.w * scalar };
    }
}

impl<T> Copy for Vector4<T> where T : std::marker::Copy {}

impl<T> std::fmt::Display for Vector4<T> where T : std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "[x: {}, y: {}, z: {}, w: {}]", self.x, self.y, self.z, self.w);
    }
}

impl<T, U> std::ops::Add<Vector4<U>> for Vector4<T> where T : std::ops::Add<U, Output = T>
{
    type Output = Vector4<T>;
    fn add(self, rhs: Vector4<U>) -> Self::Output
    {
        return Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w };
    }
}

impl<T, U> std::ops::Sub<Vector4<U>> for Vector4<T> where T : std::ops::Sub<U, Output = T>
{
    type Output = Vector4<T>;
    fn sub(self, rhs: Vector4<U>) -> Self::Output
    {
        return Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w };
    }
}

impl<T, U> std::ops::Mul<Vector4<U>> for Vector4<T> where T : std::ops::Mul<U, Output = T>
{
    type Output = Vector4<T>;
    fn mul(self, rhs: Vector4<U>) -> Self::Output
    {
        return Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w };
    }
}

impl<T, U> std::ops::Div<Vector4<U>> for Vector4<T> where T : std::ops::Div<U, Output = T>
{
    type Output = Vector4<T>;
    fn div(self, rhs: Vector4<U>) -> Self::Output
    {
        return Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w };
    }
}
