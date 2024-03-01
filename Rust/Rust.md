## Intro

1. Código compilado en código máquina, por lo que es muy rápido.
2. Tiene admisión automática de código sin recolector de basura.
3. Se eliminan las vulnerabilidades de corrupción de memoria ya que corrige estos errores en tiempo de ejecución.
4. Misma base de código para diferentes SO.

## Variables

Se declara de la siguiente forma:

```Rust
let name:type = valor;
```

Cuando una variable no es usada y es de tipo booleano se puede utilizar el nombre de variable _ para que el compilador ignore el hecho de que no está usada:

```Rust
let _success = some_function();

let _ = some_function();
```

Constantes:

```Rust
const MY_CONSTANT: i32 = 99;
```

### Mutable

Las variables pueden ser declaradas como mutables.

```Rust
let array:[i32;5] = [0; 5];
array[2] = 99; //No es válido.

let mut array[i32;5] = [0;5];
array[2] = 99;
```

## Tipos de datos:

Se consideran primitivos y se almacenan en la pila. Permite crear sus propias estructuras de datos.
1. Escalares
	 1. Valor único
	 2. Incluyen:
		 1. Int
		 2. Char
		 3. Boolean
2.  Compuestos
	1. Puede tomar múltiples valores
	2. Incluyen:
		1. Array
		2. Tuples

### Enteros

Se pueden declarar con signo o sin signo.

Declarar variable con signo de 8 bytes:
```Rust
let name : i8 = -20;
```

Declarar variable sin signo de 8 bytes:
```Rust
let name : u8 = 20;
```

### Flotante

Declarar variable de coma flotante de 32 bits:
```Rust
let name : f32 = 2.8;
```

Declarar variable de coma flotante de 64 bits:
```Rust
let name : f64 = 3.14;
```

### Char

Declarar variable char:
```Rust
let name:char = 'A';
```

### Boolean

Declarar variable booleana:
```Rust
let name:bool = true;
```

### Array

Declarar un array y definir los valores manualmente:
```Rust
let array:[dataType;size] = [data];
```

Rellenar un array de elementos de forma automática con el número que se asigne:
```Rust
let array:[dataType;size] = [0; 1000];
```

Para imprimir un elemento del array:
```Rust
println!("{}", array[position]);
```

### Tuple

Declaración:
```Rust
let tuple:(&str, &str, i32) = ("Charles", "Dickens", 1812);
```

Imprimir elementos:
```Rust
println!("{} {} was born in {}.", tuple.0, tuple.1, tuple.2);
```

#### Decostruction

Nombra los elementos de un tuple.
```Rust
let (first_name:&str, last_name:&str, dob:i32) = tuple;
println!("{} {} was born in {}.", first_name, last_name, dob);
```

### String

Hay dos tipos de strings, String y &str(slice). El String se puede transformar y el slice no. Por ello el String se almacena en el heap y el slice en el stack.

Declarar Slice:
```Rust
let slice:&str = "Charles Dickens";
```

Un slice se puede convertir en un String utilizando los métodos .to_string() o String::from(), as_str() y con &.

```Rust
let str:String = slice.to_string();
let str:String = String::form(slice);
```

De las siguientes formas no hay conversión sino que se accede a la dirección de memoria en la que está almacenado el String.
```Rust
let slice:(&str) = &str;
let slice:&str = str.as_str();
```

#### Concat strings

Hay muchos métodos:

```Rust
let full_name:String = format!("{} {}", first_name, last_name);
let full_name:String = [first_name, " ", last_name].concat();
```

Si el String puede ser modificado puedes usar el método push_str:

```Rust
let mut name:String = String::from("Rasta");
name.push_str("Mouse");
```

Se puede utilizar el operador + pero solo para concatenar un slice a un string:

```Rust
let first_name:String = String::from("Rasta");
let full_name:String = first_name + "Mouse";
```

### Casting

Es el cambio de tipo de dato, se puede hacer con la palabra reservada as:

```Rust
let int8 : u8 = 255;

let int16 : u16 = int8 as u16;
```

Se puede hacer también para convertir de un int con un número ascii en un char.

### Vector

Es una lista de valores que se debe declarar mutable para poder añadir nuevos. Se declara de la siguiente forma:

```Rust
let mut vector : Vec<u8> = Vec::new();

let mut vector : Vec<u8> = vec![];
```

Cuenta con varias funciones que pueden ser llamadas de la siguiente forma:

```Rust
let mut vector : Vec<u8> = Vec::new();

vector.push(1);

for data:&u8 in vector.iter(){
	println!("{}", data);
}
```

Se pueden insertar un elemento en una posición específica moviendo el resto de valores con .insert(). También se puede borrar elementos con la función .remove(). El método .contains() para saber si un elemento existe en el vector o no.

### Maps

Existen dos tipos de mapas, los hashmap y los BtreeMap, los segundos se utilizan cuando se quiere ordenar las claves.

Los valores del mapa se extraen introduciendo la clave con el método .get y la clave:

```Rust
let value:Option<&&str> = map.get(&3);
```

No hay sistema de colisión de claves por lo que se si añade un nuevo valor con una clave repetida esta se sobrescribirá. Para cerciorarse de que esto no ocurra se utiliza la función .contains_key() que devuelve un valor booleano.

## Operadores

Son los mismo que en todos los lenguajes de programación:

```
+ para sumar
- para restar
* para multiplicar
/ para dividir
% para el módulo
```

Operadores lógicos:

```
== para igual que
>< para mayor/menor que
>= para mayor igual que
<= para menor igual que
!= para diferente que

&& para que dos condiciones se tengan que cumplir
|| para que una de las condiciones se tenga que cumplir
```

Existen operadores binarios que cambiar los valores a nivel binario.

```
& para el and
| para el or
^ para el XOR
<< para shift a la izquierda
>> para shift a la derecha
```

## Control flow

### If/else

```Rust
if condition{
	program;
}else if condition{
	program;
}else{
	program;
}
```

Al igual que en el resto de lenguajes de programación se puede poner en la condición una variable booleana:

```Rust
let condition1:bool = false;
let condition2:bool = true;

if condition1{
	println!("{}", condition1);
}else if condition2{
	println!("{}", condition2);
}else{
	println!("No condition");
}
```

Se pueden utilizar los operadores booleanos para poner las condiciones(||, &&, ...)

Podemos usar los operadores lógicos con variables:

```Rust
let animal:&str = "Dog";

if animal == "Dog"{
	println!("Woof");
}else if animal == "Cat"{
	println!("Meow");
}else{
	println!("Unknown");
}
```

El código anterior se puede escribir también de esta forma:

```Rust
let animal:&str = "Dog";

match animal {
	"Dog" => println!("Woof");
	"Cat" => println!("Meow");
	_ => println!("Unknown");
}
```

El carácter _ sirve para que todos los casos que no cumplan las condiciones anteriores vayan a ese caso, como si fuera un else.

## Enums

Los enum sirven para almacenar diferentes casos en una sola variable que no pueden ser cambiados.

```Rust
enum Status{
	Dead,
	Alive
}

fn main(){
	let (first_name:&str, last_name:&str, status:Status) = ("Charles","Dickens", Status::Dead);
}
```

Se puede utilizar con match para los casos:

```Rust
//.. Código anterior

match status{
	Status::Alive => prinln!("{} {} está vivo", first_name, last_name);
	Status::Dead => println!("{} {} está muerto", first_name, last_name);
}
```

## Loops

### For

La sintaxis es la siguiente:

```Rust
fn template(){
	for variable_counter in range {
		code;
	}
}

fn example(){
	for i:u8 in 1..10{ //Del 1 al 9
		println!("{}", i);
	}
}

fn example2(){
	for i:u8 in 1..=10{ //Del 1 al 10
		println!("{}", i);
	}
}
```

Se suele utilizar para iterar en elementos en una colección.

```Rust
fn main(){
	let array:[i32, 5] = [1,2,3,4,5];
	for index:usize in 0..array.len() {
		println!("{}", array[index]);
	}
}

fn example(){
	let array:[i32, 5] = [1,2,3,4,5];
	for data:&i32 in array.iter(){ // Para iterar en el array
		println!("{}", array[data]);
	}
}
```

### While

```Rust
fn template(){
	let mut counter_var :i32 = 0;
	while counter_var <= range {
		println!("{}", counter);
		counter += 1;
	}
}

fn example(){
	let mut counter_var :i32 = 0;
	while 1==1 {
		if counter > 10 {
			break;
		}
		println!("{}", counter);
		counter += 1;
	}
}
```

### Rust loop

El loop de rust se ejecuta infinitamente hasta que se corta el flujo, es más óptimo que un bucle infinito while true.

```Rust
fn main(){
let mut counter = 0;
loop {
	println!("{}", counter);
	if counter >= 10 {
		break;
	}
	counter += 1;
}

}
```

## Functions

Una función es un bloque de código al que puede llamar varias veces para ahorrarnos escribir las mismas líneas de código varias veces de forma innecesaria.

Se definen de la siguiente forma:

```Rust
fn function_name(parameter1, parameter2){
	code;
}

fn return_func(parameter1, parameter2) -> returned_variable_type {
	code;
	return variable;
}

fn return_func(parameter1, parameter2) -> returned_variable_type {
	parameter1 + parameter2;
}
```

Para llamarla sólo hace falta poner el nombre de la función y sus parámetros y en caso de que retorne algo puede utilizarse como variable o introducir su valor en una.

## Input

Para ejecutar un programa con argumentos se debe introducir en la función main lo siguiente:

```Rust
use std::env::args; // Librería estándar para leer contenido

fn main(){
	let args: Vec<String> = args().collect();
	for arg:&String in args.iter() {
		println!("{}", arg);
	}
}
```
 Se ejecutaría así:
```Cmd
.\program.exe test1 test2 test3

test1
test2
test3
```

Se puede poner número mínimo de argumentos y mostrar el uso de forma automática:

```Rust
fn main(){
	let args: Vec<String> = args().collection();
	if args.len() < 3 {
		println!("No hay suficientes argumentos");
		show_usage();
		return;
	}
}

fn show_usage(){
	println!("app.exe <arg1> <arg2>");
}
```

![[Pasted image 20240227123706.png]]

Para leer archivos una vez ejecutado el programa debemos usar la librería std::io::Write con la función read_line():

```Rust
use std::io;  
use std::io::Write;  
  
fn main() {  
    loop {  
        print!("> ");  
        let _ = io::stdout().flush();  
        let mut input: String = String::new();  
        let _ = io::stdin().read_line(&mut input);  
        println!("You said: {}", input);  
    }  
}
```

1. `print!("> ");`: Esta línea imprime el prompt `>` en la salida estándar. Esto es simplemente un indicador visual para el usuario de que se espera una entrada.
    
2. `let _ = io::stdout().flush();`: `io::stdout()` proporciona una instancia de `Stdout`, que es la salida estándar de la consola. `.flush()` se utiliza para asegurarse de que la salida se envíe de inmediato a la consola en lugar de almacenarse en un búfer. El `_ =` se utiliza para ignorar cualquier posible error que pueda ocurrir durante el proceso de limpieza del búfer.
    
3. `let mut input: String = String::new();`: Esto declara una variable mutable llamada `input` que almacenará la entrada del usuario. `String::new()` crea una nueva cadena vacía que se utilizará para almacenar la entrada del usuario.
    
4. `let _ = io::stdin().read_line(&mut input);`: `io::stdin()` proporciona una instancia de `Stdin`, que es la entrada estándar de la consola. `.read_line(&mut input)` lee una línea de la entrada estándar y la almacena en la variable `input`. El `_ =` se utiliza nuevamente para ignorar cualquier posible error que pueda ocurrir durante la lectura de la entrada del usuario.
    
5. `println!("You said: {}", input);`: Esto imprime en la consola el mensaje "You said: " seguido de lo que el usuario ha ingresado, que está almacenado en la variable `input`. Esto proporciona retroalimentación al usuario mostrando lo que ingresó.

Se le puede añadir las siguientes líneas:

```Rust
if input.trim_end().eq_ignore_ascii_case("Exit"){  
    break;  
}
```

Para terminar el programa cuando el usuario ingrese exit. La función trim_end() sirve para eliminar cualquier espacio en blanco que se pueda producir después de entrar al condicional y la función eq_ignore_ascii_case() para no diferenciar entre mayus y minus.

## Closures

Los closures son funciones que sirven para finalizar un programa que sirven para terminar una función y ejecutar un código cuando esto ocurre. Se declaran de la siguiente forma:

```Rust
fn main(){
	|| {
		code;
	};
}
```

Se debe asignar a una variable:

```Rust
fn main(){
	let closure:fn() = || {
		println!("Hello from closure");
	};
	closure();
}
```

Podemos declarar parámetros para la función:

```Rust
fn main(){
	let closure:fn(&str) = |message: &str| {
		println!("{}", message);
	};
	closure("Hello World");
}
```

También podemos retornar datos de la función:

```Rust
fn main(){
	let closure:fn(&str) = |name: &str|-> String {
		println!("Hello, {}", name);
	};
	let name:String = closure("Rasta");
	println!("{}", name);
}
```

## Ownership

Es una propiedad que existe en Rust que consiste en que una variable que se almacena en el heap solo tiene valores de referencia como punteros que van hacia ese valor. Por lo que si le damos el valor de una variable directamente recibiría solo el valor del puntero y en Rust los valor a los que apuntan los punteros tienen owner por lo que ninguna otra variable puede hacer referencia a esa dirección del heap.

### Borrowing

Para solucionar esto existe el borrowing para compartir el direccionamiento en memoria sin cambiar de lugar los datos de la memoria. Así se pasa la dirección de memoria en lugar de los datos.

Se haría de la siguiente forma:

```Rust
fn main(){
	let mut message1:String = String::from("Hello world");
	println!("{}", message1);
	let message2:&String = &message1;
	println!("The message lives in {:p}", message2);
	println!("{}", message); //Printea el valor de message1 sin tener el ownership y solo haciendo referencia con un puntero.
}
```

Todo se complica cuando queremos modificar el valor del mensaje mediante message2. Al ser un puntero no se le puede introducir un valor directamente. Tendríamos que quitar la referencia, lo cual se hace con un *.

```Rust
fn main(){
	let mut message1:String = String::from("Hello world");
	println!("{}", message1);
	let message2:&String = &message1;
	*message2 = String::from("Bye world");
	println!("{}", message1); //Se imprime el nuevo valor que ha sido añadido mediante el message2
}
```

## Struct

Las estructuras son un tipo de datos multievaluado. Puede contener más de una variable y se declara de la siguiente forma:

```Rust
enum Status{
	Alive,
	Dead
}

struct Person{
	first_name: String,
	last_name: String,
	date_of_birth: u16,
	status: Status;
}
```
Y se le asignan valores de la siguiente forma:

```Rust
fn main(){
	let person = Person{
		first_name: String::from("Charles"),
		last_name: String::from("Dickens"),
		date_of_birth: 1812,
		status: Status::Alive
	};
	println!("{} {} was born in {}.", person.first_name, person.last_name, person.date_of_birth);
}
```

### Métodos en Structs

Para crear las funciones de los Structs llamados métodos tenemos que seguir la siguiente sintaxis:

```Rust
impl Person{
	fn kill(&mut self){
		self.status = Status::Dead;
	}
}

fn main(){
	let mut person = Person {
		first_name: String::from("Charles"),
		last_name: String::from("Dickens"),
		date_of_birth: 1812,
		status: Status::Alive
	};
	person.kill();
}
```

Para crear métodos que retornen valores podemos hacerlo como una función normal:

```Rust
impl Rectangle {
	fn calculate_area(&self) -> u16 {
		self.lenght * self.width //Retorna el valor de la multiplicacion
	}
}
```

El self sirve para referirse a sus propios atributos. Podemos hacer un constructor o funciones que reciban parámetros de la siguiente forma:

```Rust
struct Person{  
    name: String,  
    nick: String,  
    rol: Rol  
}  
  
impl Person {  
    fn web(&mut self){  
        self.rol = Rol::Web;  
    }  
    fn new(name: String, nick: String, rol: Rol) -> Self{  
        Self{  
            name,  
            nick,  
            rol  
        }  
    }  
}  
  
#[derive(Debug)]  
enum Rol{  
    Web,  
    Pwn,  
    Crypto,  
    Rev,  
    Stego,  
    Osint  
}  
  
fn main() {  
    let user = Person::new(String::from("Adrian"), String::from("Astharot"), Rol::web);  
      
}
```

### Traits

Los traits son equivalentes a las interfaces en Java, se crea una clase con una serie de métodos vacíos para redefinirlos más tarde su comportamiento.

```Rust
trait AnimalTraits {
	fn make_noise(&self);
}

impl AnimalTraits for Sheep { //Aqui se implementa la herencia
	fn make_noise(&self){
		println!("Baaa");
	}
}

impl AnimalTraits for Dog {
	fn make_noise(&self){
		println!("Woof");
	}
}
```

Se pueden también crear métodos que realicen una acción y se heradarán a todas las clases hijas.

## Error handling

### Arrays

Cuando especificamos el tamaño del array al declararlo de forma directa e intentamos acceder a una posición que no existe recibimos un error en tiempo de compilación, por el contrario, si no definimos su tamaño estrictamente el error saltará en tiempo de ejecución.

```Rust
fn main(){
	let array:[i32, 5] = [0;5];
	println!("{}", array[5]); //Error en tiempo de compilación
}

fn main(){
	let array:Vec<i32> = vec![0;5];
	println!("{}", array[5]); //Error en tiempo de ejecución
}
```

### Match

Para evitar este tipo de errores podemos utilizar la palabra reservada Match junto con las palabras Some y None en las que realiza una acción en el caso de encontrar un valor(Some) y otra en el caso de no encontrar(None).

```Rust
fn main(){
	let array:Vec:i32 = vec![0;5];
	for i in 0..=5 {
		let option:Option<&i32> = vector.get(i); //Guarda el valor en la posición i.
		match option {
			None => println!("No data found at index {}.", i),
			Some(d:&i32) => println!("The data at {} is {}.", i, d)
		}
	}
}
```

### Result

De otra forma se puede utilizar el enumerado result para gestionar los errores de una forma más específica. Se usa OK() y Err().

```Rust
use std::fs;

fn main(){
	let result = fs::File::open("C:\\text.txt");
	match result {
		OK(_) => println!("File opened"),
		Err(e:Error) => println!("Error opening file: {}", e);
	}
}
```

### Debugging

Algo muy útil es `dbg!` con el cual podemos recibir el valor de expresiones:

```Rust
fn main(){
	let vector = vec![1,2,3,4,5];
	dbg!(vector); //Imprime todos los valores del vector
}
```

También se puede utilizar con datos más complejos:

```Rust
use std::fs;

fn main(){
	let file_handle = fs::File::open("C:\\test.txt");
	match find_handle {
		OK(_) => println!("File openend");
		Err(e) => { dbg!(e); }
	}
}
```

## Generic Data Types

Se pueden crear arrays de tipos de datos especiales, como de structs.

```Rust
struct Person{
	...
}

fn main(){
	let array<Person> = Vec::new();
}
```

Se pueden usar tipos generales para poder retornar funciones e introducir datos. Para eso sirve la T, tipo de dato genérico.

```Rust
fn main(){
	let result = add_integers(1,2);
	println!("{}", result);
}

fn add_integers<T>(i1: T, i2: T) -> T {
	i1 + i2
}
```

Para asegurarnos que el tipo de dato retornado será el mismo que el que entra podemos hacerlo de dos forma:

```Rust
fn add_integers<T: Add<Output>>(i1: T, i2: T) -> T {
	i1 + i2
}

fn add_integers<T>(i1: T, i2: T) -> T where T: Add<Output> {
	i1 + i2
}
```

Esto bloqueará algunas acciones como introducir datos diferentes cómo parámetros.

## Threads

En una computadora moderna se pueden llevar a cabo varias tareas al mismo tiempo al contar con más de un núcleo físico. Los problemas vienen cuando varios subprocesos intentan modificar el mismo objeto. Para esto se utilizan los hilos ya que cuando un hilo accede a un recurso evita que otro lo haga.

Para crear threads usaremos thread::spawn. Lo que retorna esta función es un handle que sirve para esperar a que termine el thread de ejecutarse. Para poder ejecutar uno correctamente usaremos closures.

```Rust
fn main(){
	let handle = thread::spawn(|| {
		println!("Hello world");
	});
	let result = handle.join();
	match result {
		Ok(_) = println!("Thread finished successfully"),
		Err(_) = println!("Thread did not finish successfully")
	}
}
```

También podemos hacer que retorne un valor utilizando la palabra reservada move que obligará al closure a tener ownership del valor retornado.

```Rust
fn main(){
	let value = 20;
	let handle = thread::spawn(move|| -> i32 {
		value*2;
	});
	let result = handle.join();
	match result {
		Ok(r) => println!("Result: {}", r),
		Err(_) => println!("Thread did not finish successfully)
	}
}
```

## Channels

Los channels sirve para la comunicación entre threads, cuentan con un Sender y un Receiver con los que pueden enviar y recibir información. Su declaración es de la siguiente forma:

```Rust
use std::sync::mpsc;

fn main(){
	let (s, r) = mpsc::channel();
}
```

Puede haber varios Senders pero solo un Receiver. Para enviar información debemos hacerlo mediante hilos:

```Rust
use std::sync::mpsc;

fn main(){
	let (s, r) = mpsc::channel();
	let _ = thread.spawn(move ||
	{
		let _ = s.send("Hello from thread");
	});
	match r.recv() {
		Ok(m) => println!("{}", m),
		Err(_) => {}
	}
}
```

## Foreign Function Interface

El FFI es el acceso a funcionalidades dentro de bibliotecas C externas. Para hacking lo más común será usarlo para interactuar con la API de Windows. Se declara con la palabra reservada link llevando a la librería y añadiendo las funciones en el bloque extern como en el siguiente ejemplo en el que usa la librería de Windows kernel32:

```Rust
use std::ffi::c_void;  
use std::ptr::null_mut;

#[link(name = "kernel32")]
extern {
	fn GetLastError() -> u32;
	fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: bool, dwProcessId: u32) -> *mut c_void;
}
fn main() {  
    let handle = unsafe {  
        OpenProcess(0xF01FF, false, 26768)  
    };   
    if handle == null_mut() {  
        println!("OpenProcess failure: {}", unsafe {GetLastError()});  
    }else{  
        println!("hProcess: {:?}", handle);  
    }  
}
```
Ahora expliquemos detalladamente este código.

Utilizamos la librería de la windows api kernel 32 la cual contiene las funciones que vamos a usar GetLastError() y OpenProcess(). La primera hace un get del último error que se ha producido en la windows api y la segunda abre un proceso existente con una serie de parámetros como su PID.

Crea el handle con el nombre de la función unsafe ya que esta se marca como unsafe porque todas las funciones externas se marcan de esta forma. Si el handle falla al abrirse porque no puede abrir el proceso imprime lo que pone en el println! y el último error producido. En caso de abrirse correctamente muestra el proceso abierto.

Para evitar los errores de unsafe que aparecen podemos llamar a estas funciones desde funciones nuevas creadas y hacer todo el código dentro:

```Rust
use std::ffi::c_void;

fn open_process(dwDesiredAccess: u32, bInheritHandle: bool, dwProcessId: u32) -> Result<*mut c_void, u32> {
	let handle = unsafe {
		OpenProcess(dwDesiredAccess, bInheritHandle, dwProcessId)
	};
	if handle == null_mut() {
		Err(unsafe {GetLastError()});
	}else{
		Ok(handle)
	}
}
fn main(){
	let result = open_process(0xF01FF, false, 26768);
	match result {
		Ok(h) => { println!("hProcess: {:x?}", h) }
		Err(e) => { prinln!("OpenProcess faiulure: {}", e) }
	}
}
```

De esta forma hacemos el mismo código de antes pero evitando los errores. También podemos usarlo para convertir tipos de datos de Rust en datos nativos. Por ejemplo, algunas librerías de windows como LoadLibraryW() necesitan una cadena de carácteres unicode terminado en un byte nulo lo que corresponde en Rust `*const u16`. Para convertir los datos lo haríamos de la siguiente forma:

```Rust
fn load_library(file_name: &str) -> Result<*mut c_void, u32> {
	// COnvertir en u16
	let mut name = file_name.encode_utf16().collect::<Vec<u16>>();
	// Comprobar que acaba el byte nulo
	if *name.last().unwrap() != 0 {
		name.push(0);
	}
	//Llamada a la api
	let handle = unsafe { LoadLibraryW(name.as_ptr()) };
	//Return
	if handle == null_nut() {
		Err(unsafe { GetLastError() })
	} else {
		Ok(handle)
	}
}
```

## Modules

En Rust todo lo que pueda generar un programa en Rust como dll o exes se le llama Crates.

Se pueden crear módulos para importar al programa principal, algo parecido a librerías. Puedes crear diferentes funciones en esos módulos y luego importarlas al programa principal mientras estén en la misma carpeta.

```Rust
// En el módulo calc.rs
pub fn add(i1: i32, i2: i32){
	i1 + i2
}

pub fn subtract(i1: i32, i2: i32){
	i1 - i2
}

// En el main

mod calc;
use calc::add, calc::subtract;

fn main(){
	result = add(20, 15);
	println!("{}", result); //Imprime 35
}
```

## Dependencias

Existen muchas dependencias que no vienen en la librería estandar de Rust y que son muy útiles, hay un repositorio dónde la comunidad sube sus propias Crates para que todo el mundo las pueda utilizar.

https://crates.io/

Para poder añadir al programa nuevas Crates hay varias formas. Desde la línea de comando de su terminal de su editor de código el siguiente comando:

cargo add `Crate`

Una de las más utilizadas es rand que genera número aleatorios.

Se puede modificar también el archivo Cargo.toml de la siguiente forma:

```
[dependencies]
rand = "0.8.5"
```

Así ya se puede importar en el programa la librería y sus funciones.

```Rust
use rand::random;

fn main(){
	let rand_number = random::<u32>();
	println!("Your random number is: {}", rand_number);
}
```

## Building

Ejecutamos el código con cargo run y los binarios se almacenan en `\target\debug`.

Para una release usamos cargo build --release y se almacena en `\target\release`

