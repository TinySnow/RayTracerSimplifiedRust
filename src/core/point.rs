/// # 说明
/// 三维点的结构体。
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    /// # 说明
    /// 初始化三维点。<p>
    /// # 注意
    /// 　　此方法需要三个参数，如需一个参数初始化三个值，请使用 `Point::broadcast` 方法。<p>
    /// # 例子
    /// ```
    /// let origin = Point::new(0.0, 0.0, 0.0);
    /// let unit_x = Point::new(1.0, 0.0, 0.0);
    /// let unit_y = Point::new(0.0, 1.0, 0.0);
    /// let unit_z = Point::new(0.0, 0.0, 1.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    /// # 说明
    /// 初始化三维点。<p>
    /// # 注意
    /// 　　此方法仅需一个参数，该参数初始化三个值，x, y, z 三个坐标值均相同。<p>
    /// 　　如需三个坐标值不同，请使用需要三个参数的 `Point::new` 方法。<p>
    /// # 例子
    /// ```
    /// let origin = Point::broadcast(0.0);
    /// let all_1 = Point::broadcast(1.0);
    /// ```
    pub fn broadcast(x: f64) -> Point {
        Point { x, y: x, z: x }
    }

    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }
}