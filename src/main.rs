slint::include_modules!();

const TAX_PERCENTAGE: f64 = 0.30;
const OWNER_PERCENTAGE: f64 = 0.55;
const PROFIT_PERCENTAGE: f64 = 0.05;
const OP_PERCENTAGE: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    ui.on_divide_income(move |value| {
        let ui = ui_handle.unwrap();
        let num: f64 = value.trim().parse().unwrap();
        let tax: f64 = num * TAX_PERCENTAGE;
        let owner: f64 = num * OWNER_PERCENTAGE;
        let profit: f64 = num * PROFIT_PERCENTAGE;
        let op: f64 = num * OP_PERCENTAGE;
        let result = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpEx: {:.2}", {tax}, {owner}, {profit}, {op});
        ui.set_results(result.into());
    });

    ui.run()
}
