/* An Appraisal is a part of a Utility calculation, that can be evaluated. */
pub trait Appraisal<Context>: std::fmt::Display {
    fn evaluate(&self, context: &Context) -> f64;
    #[cfg(feature = "egui")]
    fn render(&self, context: &Context, ui: &mut egui::Ui);
}

/* A Value is a constant that can be used as part of a Utility calculation. */
pub struct Value {
    value: f64,
}

impl Value {
    pub fn new(value: f64) -> Box<Self> {
        Box::new(Self { value })
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value({:.3})", self.value)
    }
}

impl<Context> Appraisal<Context> for Value {
    fn evaluate(&self, _context: &Context) -> f64 {
        self.value
    }

    #[cfg(feature = "egui")]
    fn render(&self, _context: &Context, ui: &mut egui::Ui) {
        ui.label(format!("Value = {:.3}", self.value));
    }
}

/* A Product can multiply several Appraisals together. */
pub struct Product<Context> {
    children: Vec<Box<dyn Appraisal<Context>>>,
}

impl<Context> Product<Context> {
    pub fn new(children: Vec<Box<dyn Appraisal<Context>>>) -> Box<Self> {
        Box::new(Self { children })
    }
}

impl<Context> std::fmt::Display for Product<Context> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        let mut first = true;
        for child in &self.children {
            if first {
                ret += &format!("{}", child);
                first = false;
            } else {
                ret += &format!(", {}", child);
            }
        }
        write!(f, "Product({})", ret)
    }
}

impl<Context> Appraisal<Context> for Product<Context> {
    fn evaluate(&self, context: &Context) -> f64 {
        let mut value = 1.0;
        for child in &self.children {
            value *= child.evaluate(context);
        }
        value
    }

    #[cfg(feature = "egui")]
    fn render(&self, context: &Context, ui: &mut egui::Ui) {
        let text = format!("Product({:.3})", self.evaluate(context));
        egui::CollapsingHeader::new(text)
            .default_open(false)
            .show(ui, |ui| {
                for child in &self.children {
                    child.render(context, ui);
                }
            });
    }
}

/* A weighted sum can combine several Appraisals together, with different weights. */
pub struct WeightedSum<Context> {
    children: Vec<(f64, Box<dyn Appraisal<Context>>)>,
    normalize: bool,
}

impl<Context> WeightedSum<Context> {
    pub fn new(children: Vec<(f64, Box<dyn Appraisal<Context>>)>, normalize: bool) -> Box<Self> {
        Box::new(Self {
            children,
            normalize,
        })
    }
}

impl<Context> std::fmt::Display for WeightedSum<Context> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        let mut first = true;
        for (weight, child) in &self.children {
            if first {
                ret += &format!("{} * {}", weight, child);
                first = false;
            } else {
                ret += &format!(" + {} * {}", weight, child);
            }
        }
        write!(f, "WeightedSum({})", ret)
    }
}

impl<Context> Appraisal<Context> for WeightedSum<Context> {
    fn evaluate(&self, context: &Context) -> f64 {
        let mut sum = 0.0;
        let mut factor_sum = 0.0;
        for (weight, child) in &self.children {
            sum += weight * child.evaluate(context);
            factor_sum += weight;
        }
        // Optionally, normalize the sum into the range 0..1
        if factor_sum != 0.0 && self.normalize {
            sum /= factor_sum;
        }
        sum
    }

    #[cfg(feature = "egui")]
    fn render(&self, context: &Context, ui: &mut egui::Ui) {
        let normalize = if self.normalize { ", normalized" } else { "" };
        let text = format!("WeightedSum({:.3}{})", self.evaluate(context), normalize);
        egui::CollapsingHeader::new(text)
            .default_open(false)
            .show(ui, |ui| {
                for (weight, child) in &self.children {
                    ui.label(format!("Weight: {}", weight));
                    child.render(context, ui);
                }
            });
    }
}

/* A Lambda expression that can evaluate or fetch something within the Context. This is a leaf node. */
pub struct Lambda<F, Context>
where
    F: Fn(&Context) -> f64,
{
    func: F,
    marker: std::marker::PhantomData<Context>,
}

impl<F, Context> Lambda<F, Context>
where
    F: Fn(&Context) -> f64,
{
    pub fn new(func: F) -> Box<Self> {
        Box::new(Self {
            func,
            marker: std::marker::PhantomData,
        })
    }
}

impl<F, Context> std::fmt::Display for Lambda<F, Context>
where
    F: Fn(&Context) -> f64,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lambda")
    }
}

impl<Context, F> Appraisal<Context> for Lambda<F, Context>
where
    F: Fn(&Context) -> f64,
{
    fn evaluate(&self, context: &Context) -> f64 {
        (self.func)(context)
    }

    #[cfg(feature = "egui")]
    fn render(&self, context: &Context, ui: &mut egui::Ui) {
        ui.label(format!("Lambda(ctx) = {:.3}", self.evaluate(context)));
    }
}

/* An expression that can apply to a Utility value. */
pub struct Curve<F, Context>
where
    F: Fn(f64) -> f64,
{
    child: Box<dyn Appraisal<Context>>,
    func: F,
}

impl<Context, F> Curve<F, Context>
where
    F: Fn(f64) -> f64,
{
    pub fn new(func: F, child: Box<dyn Appraisal<Context>>) -> Box<Self> {
        Box::new(Self { child, func })
    }
}

impl<F, Context> std::fmt::Display for Curve<F, Context>
where
    F: Fn(f64) -> f64,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Curve({:.3})", self.child)
    }
}

impl<Context, F> Appraisal<Context> for Curve<F, Context>
where
    F: Fn(f64) -> f64,
{
    fn evaluate(&self, context: &Context) -> f64 {
        (self.func)(self.child.evaluate(context))
    }

    #[cfg(feature = "egui")]
    fn render(&self, context: &Context, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(format!(
            "Curve({:.3}) = {:.3}",
            self.child.evaluate(context),
            self.evaluate(context)
        ))
        .default_open(false)
        .show(ui, |ui| {
            self.child.render(context, ui);
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::*;

    #[test]
    fn eval_test() -> std::io::Result<()> {
        struct TestCtx {
            value1: f64,
            value2: f64,
            value3: f64,
        }

        // Declare a number of different contexts (these are equivalent to game states)
        let ctx1 = TestCtx {
            value1: 1.0,
            value2: 1.0,
            value3: 1.0,
        };
        let ctx2 = TestCtx {
            value1: 1.0,
            value2: 2.0,
            value3: 3.0,
        };
        let ctx3 = TestCtx {
            value1: 4.0,
            value2: 2.0,
            value3: 2.0,
        };

        // Declare a number of different Appraisals (these are equivalent to utility formulas for actions)

        // Appraisal 1: A combination of weighted sums, products, values and lambdas
        let app1 = WeightedSum::<TestCtx>::new(
            vec![
                (1.0, Value::new(5.0)),
                (2.0, Lambda::<_, TestCtx>::new(|ctx| ctx.value1)),
                (
                    3.0,
                    Product::<TestCtx>::new(vec![
                        Lambda::<_, TestCtx>::new(|ctx| ctx.value2),
                        Lambda::<_, TestCtx>::new(|ctx| ctx.value3),
                    ]),
                ),
            ],
            false,
        );
        // Appraisal 2: A curve function applied to a lambda
        let app2 =
            Curve::<_, TestCtx>::new(|val| 5.0 - val, Lambda::<_, TestCtx>::new(|ctx| ctx.value3));

        // Evaluate the different utility formulas with different contexts
        assert_eq!(app1.evaluate(&ctx1), 10.0);
        assert_eq!(app1.evaluate(&ctx2), 25.0);
        assert_eq!(app1.evaluate(&ctx3), 25.0);

        assert_eq!(app2.evaluate(&ctx1), 4.0);
        assert_eq!(app2.evaluate(&ctx2), 2.0);
        assert_eq!(app2.evaluate(&ctx3), 3.0);

        Ok(())
    }

    #[test]
    fn game_example() -> std::io::Result<()> {
        struct GameCtx {
            attack_power: f64,
            enemy_cur_health: f64,
            enemy_max_health: f64,
            enemy_threat: f64, // 0-1, with 1 being highest
        }

        // Declare a number of different contexts (these are equivalent to game states/targets)
        let ctx1 = GameCtx {
            attack_power: 25.0,
            enemy_cur_health: 65.0,
            enemy_max_health: 100.0,
            enemy_threat: 1.0,
        };
        // Same enemy, more powerful attack
        let ctx2 = GameCtx {
            attack_power: 60.0,
            enemy_cur_health: 65.0,
            enemy_max_health: 100.0,
            enemy_threat: 1.0,
        };
        // Same attack, less dangerous enemy
        let ctx3 = GameCtx {
            attack_power: 25.0,
            enemy_cur_health: 65.0,
            enemy_max_health: 100.0,
            enemy_threat: 0.1,
        };
        // Same attack, enemy closer to death
        let ctx4 = GameCtx {
            attack_power: 25.0,
            enemy_cur_health: 10.0,
            enemy_max_health: 100.0,
            enemy_threat: 1.0,
        };

        // Declare an attack evaluation Appraisal

        // Appraisal 1: A combination of weighted sums, products, values and lambdas
        let app1 = Product::<GameCtx>::new(vec![
            // The higher threat enemy should get higher priority
            Lambda::<_, GameCtx>::new(|ctx| ctx.enemy_threat),
            // The more powerful attack should get higher priority
            Lambda::<_, GameCtx>::new(|ctx| ctx.attack_power),
            // Attacks against enemies with low health should get higher priority
            Curve::<_, GameCtx>::new(
                // Invert curve so that low % has higher priority
                |value| 1.0 - value,
                // Calculate how much life the enemy has in %
                Lambda::<_, GameCtx>::new(|ctx| ctx.enemy_cur_health / ctx.enemy_max_health),
            ),
        ]);

        // Evaluate the different utility formulas with different contexts
        // In this example here, each context represents a particular attack + enemy combination
        // Higher utility is a "better" attack
        assert_eq!(app1.evaluate(&ctx1), 8.75); // Baseline
        assert_eq!(app1.evaluate(&ctx2), 21.0); // More powerful attack = better
        assert_eq!(app1.evaluate(&ctx3), 0.875); // Less dangerous enemy = lower priority
        assert_eq!(app1.evaluate(&ctx4), 22.5); // Enemy closer to death = better

        Ok(())
    }
}
