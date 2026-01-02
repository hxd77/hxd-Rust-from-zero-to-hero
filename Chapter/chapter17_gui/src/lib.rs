//定义一个带有draw方法的trait Draw
pub trait Draw{
    fn draw(&self);
}

//定义一个Screen结构体的定义，它带有一个字段components
pub struct Screen{
    pub components: Vec<Box<dyn Draw>>, //dyn只在乎实现了trait特征
}

//在Screen上实现了一个run方法
impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

//定义一个Button结构体
pub struct Button{
    pub width:u32,
    pub height:u32,
    pub label:String,
}

//定义Button的impl方法来实现Draw
impl Draw for Button{
    fn draw(&self) {
        println!("绘制按钮: {:?}",self.label);
    }
}

