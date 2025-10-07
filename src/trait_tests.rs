#[allow(dead_code)]
trait Draw {
    fn draw(&self);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Circle {
    radius: f64,
}
impl Draw for Circle {
    fn draw(&self) {
        println!("绘制圆形：半径={}", self.radius);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Square {
    side: f64,
}
impl Draw for Square {
    fn draw(&self) {
        println!("绘制正方形：边长={}", self.side);
    }
}

#[allow(dead_code)]
struct Canvas<T>
where
    T: Draw,
{
    item: T,
}
#[allow(dead_code)]
impl<T> Canvas<T>
where
    T: Draw,
{
    fn render(&self) {
        self.item.draw();
    }
}

#[allow(dead_code)]
struct DynCanvas {
    items: Vec<Box<dyn Draw>>,
}
#[allow(dead_code)]
impl DynCanvas {
    fn render_all(&self) {
        for item in &self.items {
            item.as_ref().draw();
        }
    }
}

#[allow(dead_code)]
trait Transformer {
    type Input;
    type Output;

    fn transform(&self, input: Self::Input) -> Self::Output;
}
#[allow(dead_code)]
struct DoubleT;
#[allow(dead_code)]
struct SquareT;

impl Transformer for DoubleT {
    type Input = i32;
    type Output = i32;

    fn transform(&self, input: Self::Input) -> Self::Output {
        input * 2
    }
}

impl Transformer for SquareT {
    type Input = i32;
    type Output = i32;

    fn transform(&self, input: Self::Input) -> Self::Output {
        input * input
    }
}

#[allow(dead_code)]
struct Holder<'a, T>
where
    T: std::fmt::Debug,
{
    value: &'a T,
}
#[allow(dead_code)]
impl<'a, T> Holder<'a, T>
where
    T: std::fmt::Debug,
{
    fn show(&self) {
        println!("{:?}", self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::{Canvas, Circle, DoubleT, Draw, DynCanvas, Holder, Square, SquareT, Transformer};

    #[test]
    fn test_trait() {
        fn draw_all(v: Vec<&dyn Draw>) {
            for item in v {
                item.draw();
            }
        }

        let c = Circle { radius: 10. };
        let s = Square { side: 10. };
        let v: Vec<&dyn Draw> = vec![&c, &s];

        draw_all(v);

        fn draw_all_2(v: Vec<Box<dyn Draw>>) {
            for item in v {
                item.draw();
            }
        }
        let v: Vec<Box<dyn Draw>> = vec![Box::new(c), Box::new(s)];
        draw_all_2(v);
        // println!("{c:?}, {s:?}"); // ❌ 编译错误：c, s 已经失效。
    }

    #[test]
    fn test_trait_constraints_in_fn() {
        fn draw_item<T>(item: &T)
        where
            T: Draw,
        {
            item.draw();
        }

        let c = Circle { radius: 10. };
        draw_item(&c);
    }

    #[test]
    fn test_trait_constraint_in_struct() {
        let c = Circle { radius: 10. };
        let canvas = Canvas { item: c };

        canvas.render();
    }

    #[test]
    fn test_trait_associated_types() {
        let double = DoubleT;
        let square = SquareT;

        fn apply_transform<T, U>(value: T, transformer: &U) -> U::Output
        where
            U: Transformer<Input = T>,
        {
            transformer.transform(value)
        }

        assert_eq!(apply_transform(12, &double), 24);
        assert_eq!(apply_transform(8, &square), 64);
    }

    #[test]
    fn test_trait_dynamic_polymorphism() {
        let double = DoubleT;
        let square = SquareT;
        let transformers: Vec<Box<dyn Transformer<Input = i32, Output = i32>>> =
            vec![Box::new(double), Box::new(square)];

        let values: Vec<_> = transformers.iter().map(|t| t.transform(10)).collect();

        assert_eq!(values, vec![20, 100]);
    }

    #[test]
    fn test_trait_in_struct() {
        let c = Circle { radius: 10. };
        let s = Square { side: 12. };
        let canvas = DynCanvas {
            items: vec![Box::new(c), Box::new(s)],
        };

        canvas.render_all();
    }

    #[test]
    fn test_trait_bound_with_lifetime() {
        let name = "Alice".to_string();
        let holder = Holder { value: &name };

        holder.show();
    }
}
