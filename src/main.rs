mod perceptron;
/*
--mod para el dataFrame-- 

--mod para la funcion de generacion de valores-- (se debe crear una funcion 
que inicialice valores en rango y cantidad requerida)

el contructor del perceptron recibira el dataFrame de entrenamiento
e internamente en base a el usara la funcion de generar valores para inicializar 
los pesos en cantidad y rangos requeridos , el bias tendra un valor por defecto pero 
podra modificarse si se desea

*/

fn main() {
    let p = perceptron::Perceptron::new(10);
    p.show();
}
