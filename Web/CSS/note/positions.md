# Adding Dimensions

+ height controls the text height,not the padding height

```css
.post-img{
    width:100%;
    height:auto;
}
```

which means you can change the photo's size according to the real browser size.

+ if you want to choose 50%

it will be better to add a <div> under the <body> 

```css
.container{
    width:700px;/*which controls the width of the text*/
    margin-left:auto;
    margin-right:auto;
    /*或者
    margin:0 auto;
    实现正中效果
    */
}
```

+ adding display

```css
.p{
    display:inline;/*只占所需的space，不会撑开父元素*/
    display:block;/*单独换行，width等有效*/
    display:inline-block;/*可以使用width和height,但不会自动换行*/
}
```

+ normal flow && absolute positioning

```css
button like {
    font-size:22px;
    padding:20px;
    cursor:pointer;
    
    position:absolute;
    bottom:0;
    right:0;
    
}
```

absolute 相对的是第一个父元素使用的。

```css
button{
    position:fixed;
    z-index:1;
}
```

fixed can fix the button on the same position, instead of changing with the control.

z-index  can control the array order.

+ pseudo-elements

```css
h1::first-letter{
	font-style:normal;
    margin-right:5px;
}
h1 + p::first-line{
    color:red;
}/*only the line after the h1*/
```

```css
h2::after {
    content:"TOP";
    background-color: yellow;
    font-size: 16px;
    font-weight: bold;
    display: inline-block;
    padding:5px 10px;
}
```

