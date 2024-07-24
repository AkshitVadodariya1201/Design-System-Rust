# TODOs





## References

- [Checkbox => Specs](https://m3.material.io/components/checkbox/specs)
- [Checkbox => Default values (enabled state)](https://m3.material.io/components/checkbox/specs#e5e04d30-10bc-4297-a67a-c38bcece5e56)
- [Checkbox states](https://m3.material.io/components/checkbox/specs#2d0fbe63-7266-41c4-b74b-1a175491f336)
- [Checkbox measurements](https://m3.material.io/components/checkbox/specs#19f3bd71-946f-4782-8362-fd8338ab7f4b)
[component css](../../tokens/css/components/checkbox.css)
[baseline css](../../tokens/css/baseline.css)


<table>

<thead>

<tr>

<th>State</th>

<th>Element</th>

<th>Design Atribute</th>

<th>Role</th>

<th>Token or value</th>

</tr>

</thead>

<tbody>

<tr>

<th rowspan="14">

1\. Enabled

</th>

<td>

Container

</td>

<td>

Width

</td>

<td>

-

</td>

<td>

18dp

</td>

</tr>

<tr>

<td></td>

<td>

Height

</td>

<td>

-

</td>

<td>

18dp

</td>

</tr>

<tr>

<td></td>

<td>

Shape

</td>

<td>

-

</td>

<td>

2dp

</td>

</tr>

<tr>

<td>

Container (unselected)

</td>

<td>

Outline color

</td>

<td>

On surface

</td>

<td>

[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

2dp

</td>

</tr>

<tr>

<td>

Container (selected)

</td>

<td>

Color

</td>

<td>

Primary

</td>

<td>

[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

0dp

</td>

</tr>

<tr>

<td>

Icon

</td>

<td>

Size

</td>

<td>

-

</td>

<td>

18dp

</td>

</tr>

<tr>

<td>

Icon (selected)

</td>

<td>

Color

</td>

<td>

On primary

</td>

<td>

[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td>

Container (unselected, error)

</td>

<td>

Outline color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td>

Container (selected, error)

</td>

<td>

Color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td>

Icon (selected, error)

</td>

<td>

Color

</td>

<td>

On error

</td>

<td>

[md.sys.color.on-error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td>

State-Layer

</td>

<td>

Size

</td>

<td>

-

</td>

<td>

40dp

</td>

</tr>

<tr>

<td></td>

<td>

Shape

</td>

<td>

Full

</td>

<td>

[md.sys.shape.corner.full](/m3/pages/shape/shape-scale-&-tokens#a830b8f8-f15b-4be9-8b2e-839f78481cf2)

</td>

</tr>

<tr>

<th rowspan="7">

2\. Disabled

</th>

<td>

Container (unselected)

</td>

<td>

Outline color

</td>

<td>

On surface

</td>

<td>

[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

2dp

</td>

</tr>

<tr>

<td></td>

<td>

Opacity

</td>

<td>

-

</td>

<td>

38%

</td>

</tr>

<tr>

<td>

Container (selected)

</td>

<td>

Color

</td>

<td>

On surface

</td>

<td>

[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Opacity

</td>

<td>

-

</td>

<td>

38%

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

0dp

</td>

</tr>

<tr>

<td>

Icon (selected)

</td>

<td>

Color

</td>

<td>

Surface

</td>

<td>

[md.sys.color.surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<th rowspan="16">

3\. Hovered

</th>

<td>

State Layer (selected)

</td>

<td>

Color

</td>

<td>

Primary

</td>

<td>

[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Opacity

</td>

<td>

State layer opacity

</td>

<td>

[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states/tab%201#91c9418d-963f-4f44-b9a7-def398b58ba9)

</td>

</tr>

<tr>

<td>

State Layer (unselected)

</td>

<td>

Color

</td>

<td>

On surface

</td>

<td>

[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Opacity

</td>

<td>

State layer opacity

</td>

<td>

[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states/tab%201#91c9418d-963f-4f44-b9a7-def398b58ba9)

</td>

</tr>

<tr>

<td>

Container (unselected)

</td>

<td>

Outline color

</td>

<td>

On surface

</td>

<td>

[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

2dp

</td>

</tr>

<tr>

<td>

Container (selected)

</td>

<td>

Color

</td>

<td>

Primary

</td>

<td>

[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

0dp

</td>

</tr>

<tr>

<td>

Icon (selected)

</td>

<td>

Color

</td>

<td>

On primary

</td>

<td>

[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td>

State Layer (error)

</td>

<td>

Color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Opacity

</td>

<td>

State layer opacity

</td>

<td>

[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states/tab%201#91c9418d-963f-4f44-b9a7-def398b58ba9)

</td>

</tr>

<tr>

<td>

Container (unselected, error)

</td>

<td>

Outline color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

2dp

</td>

</tr>

<tr>

<td>

Container (selected, error)

</td>

<td>

Color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

0dp

</td>

</tr>

<tr>

<td>

Icon (selected, error)

</td>

<td>

Color

</td>

<td>

On error

</td>

<td>

[md.sys.color.on-error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<th rowspan="15">

4\. Focused

</th>

<td>

State Layer

</td>

<td>

Color

</td>

<td>

Primary

</td>

<td>

[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Opacity

</td>

<td>

State layer opacity

</td>

<td>

[md.sys.state.focus.state-layer-opacity](/m3/pages/interaction-states/tab%201#91c9418d-963f-4f44-b9a7-def398b58ba9)

</td>

</tr>

<tr>

<td>

State Layer (unselected)

</td>

<td>

Color

</td>

<td>

On surface

</td>

<td>

[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Opacity

</td>

<td>

State layer opacity

</td>

<td>

[md.sys.state.focus.state-layer-opacity](/m3/pages/interaction-states/tab%201#91c9418d-963f-4f44-b9a7-def398b58ba9)

</td>

</tr>

<tr>

<td>

Container (unselected)

</td>

<td>

Outline color

</td>

<td>

On surface

</td>

<td>

[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

2dp

</td>

</tr>

<tr>

<td>

Container (selected)

</td>

<td>

Color

</td>

<td>

Primary

</td>

<td>

[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

0dp

</td>

</tr>

<tr>

<td>

Icon (selected)

</td>

<td>

Color

</td>

<td>

On primary

</td>

<td>

[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td>

State Layer (error)

</td>

<td>

Color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td>

Container (unselected, error)

</td>

<td>

Outline color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

2dp

</td>

</tr>

<tr>

<td>

Container (selected, error)

</td>

<td>

Color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

0dp

</td>

</tr>

<tr>

<td>

Icon (selected, error)

</td>

<td>

Color

</td>

<td>

On error

</td>

<td>

[md.sys.color.on-error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<th rowspan="16">

5\. Pressed

</th>

<td>

State Layer (unselected)

</td>

<td>

Color

</td>

<td>

Primary

</td>

<td>

[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Opacity

</td>

<td>

State layer opacity

</td>

<td>

[md.sys.state.pressed.state-layer-opacity](/m3/pages/interaction-states/tab%201#91c9418d-963f-4f44-b9a7-def398b58ba9)

</td>

</tr>

<tr>

<td>

Container (unselected)

</td>

<td>

Outline color

</td>

<td>

On surface

</td>

<td>

[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

2dp

</td>

</tr>

<tr>

<td>

State Layer (selected)

</td>

<td>

Color

</td>

<td>

On surface

</td>

<td>

[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Opacity

</td>

<td>

State layer opacity

</td>

<td>

[md.sys.state.pressed.state-layer-opacity](/m3/pages/interaction-states/tab%201#91c9418d-963f-4f44-b9a7-def398b58ba9)

</td>

</tr>

<tr>

<td>

Container (selected)

</td>

<td>

Color

</td>

<td>

Primary

</td>

<td>

[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

0dp

</td>

</tr>

<tr>

<td>

Icon (selected)

</td>

<td>

Color

</td>

<td>

On primary

</td>

<td>

[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td>

State Layer (error)

</td>

<td>

Color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Opacity

</td>

<td>

State layer opacity

</td>

<td>

[md.sys.state.pressed.state-layer-opacity](/m3/pages/interaction-states/tab%201#91c9418d-963f-4f44-b9a7-def398b58ba9)

</td>

</tr>

<tr>

<td>

Container (unselected, error)

</td>

<td>

Outline color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

2dp

</td>

</tr>

<tr>

<td>

Container (selected, error)

</td>

<td>

Color

</td>

<td>

Error

</td>

<td>

[md.sys.color.error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

<tr>

<td></td>

<td>

Outline width

</td>

<td>

-

</td>

<td>

0dp

</td>

</tr>

<tr>

<td>

Icon (selected, error)

</td>

<td>

Color

</td>

<td>

On error

</td>

<td>

[md.sys.color.on-error](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)

</td>

</tr>

</tbody>

</table>