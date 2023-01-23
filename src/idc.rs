pub trait OperatorComponent {
    fn first(&self) -> &IdsComponent;
    fn last(&self) -> &IdsComponent;
    fn len(_: &Self) -> u8
    where
        Self: Sized;
}

impl core::fmt::Debug for dyn OperatorComponent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "OperatorComponent{{{:?}, {:?}}}",
            self.first(),
            self.last()
        )
    }
}

#[derive(Debug)]
pub enum IdsComponent {
    Character(String),
    OperatorComponent(Box<dyn OperatorComponent>),
}
/**
编码	字符	意义	例字	序列	例字	序列
U+2FF0	⿰	两个部件由左至右组成	相	⿰木目	𠁢	⿰丨㇍
U+2FF1	⿱	两个部件由上至下组成	杏	⿱木口	𠚤	⿱𠂊丶
U+2FF2	⿲	三个部件由左至右组成	衍	⿲彳氵亍	𠂗	⿲丿夕乚
U+2FF3	⿳	三个部件由上至下组成	京	⿳亠口小	𠋑	⿳亼目口
U+2FF4	⿴	两个部件由外而内组成	回	⿴囗口	𠀬	⿴㐁人
U+2FF5	⿵	三面包围，下方开口	凰	⿵几皇	𧓉	⿵齊虫
U+2FF6	⿶	三面包围，上方开口	凶	⿶凵㐅	义	⿶乂丶
U+2FF7	⿷	三面包围，右方开口	匠	⿷匚斤	𧆬	⿷虎九
U+2FF8	⿸	两面包围，两个部件由左上至右下组成	病	⿸疒丙	𤆯	⿸耂火
U+2FF9	⿹	两面包围，两个部件由右上至左下组成	戒	⿹戈廾	𢧌	⿹或壬
U+2FFA	⿺	两面包围，两个部件由左下至右上组成	超	⿺走召	𥘶	⿺礼分
U+2FFB	⿻	两个部件重叠	巫	⿻工从	𣏃	⿻木⿻コ一
*/

#[derive(Debug)]
pub enum BinaryOperator {
    LeftToRight,
    AboveToBelow,
    FullSurround,
    SurroundFromAbove,
    SurroundFromelow,
    SurroundFromLeft,
    SurroundFromUpperLeft,
    SurroundFromUpperRight,
    SurroundFromLowerLeft,
    Overlaid,
}

#[derive(Debug)]
pub enum TrinaryOperator {
    LeftToMiddleAndRight,
    AboveToMiddleAndBelow,
}

#[derive(Debug)]
pub struct BinaryOperatorComponent {
    pub op: BinaryOperator,
    pub parts: [Box<IdsComponent>; 2],
}

impl OperatorComponent for BinaryOperatorComponent {
    fn first(&self) -> &IdsComponent {
        &self.parts[0]
    }
    fn last(&self) -> &IdsComponent {
        &self.parts[1]
    }
    fn len(_: &Self) -> u8 {
        2
    }
}

#[derive(Debug)]
pub struct TrinaryOperatorComponent {
    pub op: TrinaryOperator,
    pub parts: [Box<IdsComponent>; 3],
}

impl OperatorComponent for TrinaryOperatorComponent {
    fn first(&self) -> &IdsComponent {
        &self.parts[0]
    }
    fn last(&self) -> &IdsComponent {
        &self.parts[2]
    }
    fn len(_: &Self) -> u8 {
        3
    }
}

fn main() {
    let a1 = IdsComponent::Character("亻".into());
    let a2 = IdsComponent::Character("俞".into());
    let a = BinaryOperatorComponent {
        op: BinaryOperator::LeftToRight,
        parts: [Box::new(a1), Box::new(a2)],
    };
    let a = IdsComponent::OperatorComponent(Box::new(a));
    let b = IdsComponent::Character("着".into());
    let c = IdsComponent::Character("乐".into());
    let e: IdsComponent;
    e = IdsComponent::OperatorComponent(Box::new(TrinaryOperatorComponent {
        op: TrinaryOperator::AboveToMiddleAndBelow,
        parts: [Box::new(a), Box::new(b), Box::new(c)],
    }));
    dbg!(e);
}
