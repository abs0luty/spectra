# Spectra programming language

Programming language made for my tutorial videos.

## Syntax

Declaring a variable:

```
var a = 3;
```

Function expression:

```
var a = fun () => {

};
```

If expression:

```
var a = if a == 3 {
	println("hello?");
} else {
	println("hello world");
};
```

While expression:

```
while true {
	println("HELLO!");
}
```

Classes:

```
class Point {
	x, 
	y,
	constructor (x, y) => {
		this.x = x;
		this.y = y;
	}
	distance_from (other_point) => { todo(); }
}
```

