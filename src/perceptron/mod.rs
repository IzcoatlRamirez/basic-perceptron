/*los datos de entrenamiento son un dataFrame de la forma:
data = pd.DataFrame({
    'Age': ages,
    'Cholesterol': cholesterol_levels,
    'Systolic_BP': systolic_bp,
    'BMI': bmi,})

podemos usar la funcion de inicializar pesos para generar cada valor
del dataFrame en cantidad y rangos requeridos

seran 4 pesos que se iran ajustando en el entrenamiento 
al recorrer cada valor en cada caracteristica del dataFrame 
*/
pub struct Perceptron {
    weights: Vec<f64>,
    bias: f64,
    //data: Dataframe,
}
/*
#debe existir internamente la funcion train que tome los datos de entrenamiento para realizar el ajuste de los pesos 
#debe existir internamente la funcion predict que tome los datos de entrada y devuelva la salida del perceptron
*/


impl Perceptron {

    /* */
    pub fn new(input_size: usize) -> Perceptron {
        let weights = vec![0.0; input_size];
        let bias = 0.0;
        Perceptron { weights, bias }
    }

    pub fn show(&self) {
        println!("Weights: {:?}", self.weights);
        println!("Bias: {}", self.bias);
    }
}