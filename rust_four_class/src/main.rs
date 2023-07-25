mod base_number;
mod point_add;
mod vec_enum_demo;
mod vec_trait_demo;

fn main() {
    // 基本数据类型相加
    base_number::run();

    // 坐标相加
    point_add::run();
    // 通过枚举方式实现vec
    vec_enum_demo::run();
    // 通过trait方式实现vec
    vec_trait_demo::run();
}
