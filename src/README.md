# Introduction

Summarize important concepts from the book.

<!-- toc -->

- [Chapter 1](#chapter-1)
- [Chapter 4](#chapter-4)
  * [Translate](#translate)
  * [Scaling](#scaling)
  * [Rotation](#rotation)
  * [Shearing](#shearing)
- [Chapter 5](#chapter-5)
  * [Transforming ray and spheres](#transforming-ray-and-spheres)
- [Chapter 6](#chapter-6)
  * [Transforming normals](#transforming-normals)
  * [Reflecting vectors](#reflecting-vectors)
  * [The Phong Reflection Model](#the-phong-reflection-model)
- [Chapter 7](#chapter-7)
- [Chapter 8](#chapter-8)

<!-- tocstop -->

## Chapter 1

Each tuple has 4 components- *x*, *y*, *z* and *w*.

$w = 1$ for a point

$w = 0$ for a vector

Thus, subtracting two points gives a vector.

## Chapter 4

We can create transformation matrices to **translate**, **scale** and **rotate** points and vectors.
It is done by multiplying the transformation matrix with the point or the vector-
$$ \text{transformation\_matrix} * \text{vector} $$

We can also chain transformations matrices and result would be the same as if they were applied to the vector one by one-
$$ \text{transformation\_matrix}_2 * \text{transformation\_matrix}_1 * \text{vector} $$

### Translate

It moves a point.
Vectors are not affected as it is just an arrow. Translating does not change its length or direction.

$$
translation(x, y, z) =
\begin{bmatrix}
1 & 0 & 0 & x \\
0 & 1 & 0 & y \\
0 & 0 & 1 & z \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

### Scaling

Move a point by multiplying it by a value.
For vectors, this transformation changes their length.

*Reflection* can be done by this transformation by multiplying the desired component by -1.

$$
scaling(x, y, z) =
\begin{bmatrix}
x & 0 & 0 & 0 \\
0 & y & 0 & 0 \\
0 & 0 & z & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

### Rotation

We can rotate a point or a point around *x*, *y* or *z* axis by a particular value of radians.
Point moves towards the direction of the fingers in the left-hand thumb rule, thumb points towards the positive side of the axis of the rotation.

$$ radians(deg) = \frac{deg}{180} \pi $$

$$
rotation_x(r) =
\begin{bmatrix}
1 & 0 & 0 & 0 \\
0 & \cos(r) & -\sin(r) & 0 \\
0 & \sin(r) & \cos(r) & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

$$
rotation_y(r) =
\begin{bmatrix}
\cos(r) & 0 & \sin(r) & 0 \\
0 & 1 & 0 & 0 \\
-\sin(r) & 0 & \cos(r) & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

$$
rotation_z(r) =
\begin{bmatrix}
\cos(r) & -\sin(r) & 0 & 0 \\
\sin(r) & cos(r) & 0 & 0 \\
0 & 0 & 1 & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

### Shearing

Shearing transformation changes each component of a tuple in proportion to the other components. So, the *x* component changes in proportion to *y* and *z*, *y* changes in proportion to *x* and *z*, and *z* changes in proportion to *x* and *y*.

$$
shearing(x_y, x_z, y_x, y_z, z_x, z_y) =
\begin{bmatrix}
1 & x_y & x_z & 0 \\
y_x & 1 & y_z & 0 \\
z_x & z_y & 1 & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

## Chapter 5

**Ray casting** is the process of creating a ray, or line, and finding the intersections of that ray with the objects in a scene.

**Hit** will always be the intersection of ray with the lowest non-negative *t* (timestamp) value.

### Transforming ray and spheres

Until now our sphere is always at origin and is of radius 1. To compute intersections of other sphere locations and sizes, we need to change our intersection algorithm. To prevent doing so, we can transform the ray itself assuming the sphere is the default one.
The transformations which moves the default sphere to interesting locations and sizes, we can apply the inverse of that to both of the ray's components.

Another way to think it is that transformations converts points between two coordinate systems-

1) At the scene level, everything is in the *world space coordinates* relative to the overall world
2) At the object level, everything is in *object space coordinates* relative to the object itself. Like default sphere being at origin and of radius 1.

Now, to intersect a ray in world space with a sphere in object space, we just convert the ray's origin and direction to that same object space.

## Chapter 6

When *P* is the point where our ray intersects an object, there are four vectors defined-

1) *E* is the *eye vector* pointing from *P* to the origin of the ray (usually where the eye exists that is looking at the scene).
2) *L* is the *light vector* pointing from *P* to the position of the light source.
3) *N* is the *surface normal*, vector that is perpendicular to the surface at *P*.
4) *R* is the *reflection vector* pointing in the direction that incoming light would bounce or reflect.

**Surface normal** is a vector perpendicular to the surface at a given point. It is a normalized vector.

### Transforming normals

After finding the normal to a point in object space, we need to transform it to the world space for it to be useful.
Multiplying it by the sphere transformation matrix will not work as it will not be perpendicular to the surface anymore.
To make it perpendicular again, we need to multiply it by the inverse transpose of the transformation matrix.

Derivation:
[Link](https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry/transforming-normals.html)

Summary of the above derivation-

Let $M$ be the transformation matrix of the sphere and $n$ be the normal vector and $v$ be the tangent to the normal.

$$ n^{T}*v = 0 $$
$$ n^{T} * M^{-1} * M * v = 0 $$
$$ (M^{-1T} * n)^{T} * (M * v) = 0 $$
$$ n'^{T} * v' = 0 $$

Where $n' = M^{-1T} * n$ and $v' = M * v$.

This means that, transforming a point on the surface of the sphere by $M$ and transforming the normal by $M^{-1T}$ will make the normal perpendicular to the surface in the world space.

### Reflecting vectors

Vector reflection is the process of finding the direction of a vector after it bounces off a surface.
Derivation Link: [Link](https://www.bluebill.net/vector_reflection.html).

### The Phong Reflection Model

An algorithm to simulate the way light interacts with surfaces. It has three different types of lighting-

1) **Ambient reflection**- light reflected from other objects in the environment. This model treats this as a constant, coloring all points equally.
2) **Diffuse reflection**- light reflected from a surface. It depends only on the angle between the light vector and the surface normal.
3) **Specular reflection**- is the reflection of the light source itself and results in **specular highlight** (the bright spot on a curved surface). It depends only on the angle between the reflection vector and the eye vector and is controlled by a parameter called **shininess**. Higher the shininess, the smaller and tighter the specular highlight.

It is basically the sum of these three reflection components.

## Chapter 7

**View Transformation** is a transformation matrix that orients the world relative to the eye.

It is easier to imagine that it moves the eyes not the world.

## Chapter 8

If some point lies in the shadow, then, the diffuse component and the specular component of the Phong reflection model should be 0. As those components depends on the light source.

To find whether a point is in a shadow, we can cast a shadow ray from the point to the light source. If the ray intersects any object in between, then the point is in the shadow.

*Acne*- Due to rounding of floating point numbers, the shadow ray may intersect the same object it is cast from. As a result, it causes the sphere to cast a shadow on its own point of intersection.
To prevent this, we can move the origin of the shadow ray by a small amount along the normal of the point.
