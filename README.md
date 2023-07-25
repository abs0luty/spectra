# Spectra programming language

Programming language made for my tutorial videos.

## Syntax

Declaring a variable:

```
var a = 3;
```

Function expression:

```
var even = fun (n) => {
	n % 2 == 0
};
```

If expression:

```
print("number a is " + if a % 2 == 0 {
	"even"
} else {
	"odd"
});
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
	distance_from (other_point) => { todo() }
}
```

