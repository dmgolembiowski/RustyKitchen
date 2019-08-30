//std::fmt;

trait OutlinePrint {
    fn pprint(&self) {
        let output = format!("
        **********
        *        *
        * ({x_value}, {y_value}) *
        *        *
        **********", x_value=self.x, y_value=self.y);


struct CartesianCoordinate {
    x: String,
    y: String,
}

impl OutlinePrint for CartesianCoordinate;

fn main() {
    let cc = CartesianCoordinate(x:"12", y:"13");
    cc.pprint(cc.x, cc.y);
}
