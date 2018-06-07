`jsonpp`
================

JSON **p**retty **p**rint.

This program pretty prints JSON. It supports customization of output.

### Installation

    cargo install jsonpp

### Examples

    % curl https://gist.githubusercontent.com/flo-l/be7177f7f29a3b1299d95f9a5b211019/raw/901b633f1f06f9cfb133143964ba874b03167b01/ugly.json | jsonpp > pretty.json
    % cat ugly.json | jsonpp > pretty.json
    % jsonpp ugly.json > pretty.json

`ugly.json`
```json
{"widget":{"debug":"on","window":{"title":"Sample Konfabulator Widget","name":"main_window","width":500,"height":500},"image":{"src":"Images/Sun.png","name":"sun1","hOffset":250,"vOffset":250,"alignment":"center"},"text":{"data":"Click Here","size":36,"style":"bold","name":"text1","hOffset":250,"vOffset":100,"alignment":"center","onMouseUp":"sun1.opacity = (sun1.opacity / 100) * 90;"}}}
```

`pretty.json`
```json
{
  "widget": {
    "debug": "on",
    "image": {
      "alignment": "center",
      "hOffset": 250,
      "name": "sun1",
      "src": "Images/Sun.png",
      "vOffset": 250
    },
    "text": {
      "alignment": "center",
      "data": "Click Here",
      "hOffset": 250,
      "name": "text1",
      "onMouseUp": "sun1.opacity = (sun1.opacity / 100) * 90;",
      "size": 36,
      "style": "bold",
      "vOffset": 100
    },
    "window": {
      "height": 500,
      "name": "main_window",
      "title": "Sample Konfabulator Widget",
      "width": 500
    }
  }
}
```

### License

See [LICENSE](LICENSE).
