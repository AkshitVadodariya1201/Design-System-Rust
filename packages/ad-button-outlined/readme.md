# TODOs

- elevation
- border radius not work


# References


- [Common Button -> Specs](https://m3.material.io/components/buttons/specs)
- [Outlined button](https://m3.material.io/components/buttons/specs#de72d8b1-ba16-4cd7-989e-e2ad3293cf63)
- [Default values (enabled state)](https://m3.material.io/components/buttons/specs#abb27683-13c8-4eb2-bfde-cbed97f7f1d5)
- [Outlined button states](https://m3.material.io/components/buttons/specs#2a15bc61-b3eb-44da-92db-08004a57c119)
- [Outlined button measurements](https://m3.material.io/components/buttons/specs#309d928e-e9ef-41dd-89fc-9bc51f78709c)
- [baseline css](../../tokens/css/baseline.css)
- [component css](../../tokens/css/components/outlined-button.css)



<table>

<thead>

<tr>

<th>State</th>

<th>Element</th>

<th>Design attribute</th>

<th>

<div>Role</div>

</th>

<th>Token or value</th>

</tr>

</thead>

<tbody>

<tr>

<th rowspan="10">1\. Enabled</th>

<td>Container</td>

<td>Outline color</td>

<td>Outline</td>

<td>[md.sys.color.outline](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td> </td>

<td>Outline width</td>

<td>-</td>

<td>1dp</td>

</tr>

<tr>

<td> </td>

<td>Elevation</td>

<td>Level 0</td>

<td>[md.sys.elevation.level0](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td> </td>

<td>Font</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.font](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td> </td>

<td>Line height</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.line-height](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td> </td>

<td>Size</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.size](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td> </td>

<td>Tracking</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.tracking](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td> </td>

<td>Weight</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.weight](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="6">2\. Disabled</th>

<td>Container</td>

<td>Outline color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Outline opacity</td>

<td>-</td>

<td>12%</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>-</td>

<td>38%</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>-</td>

<td>38%</td>

</tr>

<tr>

<th rowspan="5">3\. Hovered</th>

<td>Container</td>

<td>Outline color</td>

<td>Outline</td>

<td>[md.sys.color.outline](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>State layer color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>State layer opacity</td>

<td>Hover state layer opacity</td>

<td>[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="5">4\. Focused</th>

<td>Container</td>

<td>Outline color</td>

<td>Primary</td>

<td>md.sys.color.primary</td>

</tr>

<tr>

<td></td>

<td>State layer color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>State layer opacity</td>

<td>Focus state layer opacity</td>

<td>[md.sys.state.focus.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="5">5\. Pressed</th>

<td>Container</td>

<td>Outline color</td>

<td>Outline</td>

<td>[md.sys.color.outline](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>State layer color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>State layer opacity</td>

<td>Pressed state layer opacity</td>

<td>[md.sys.state.pressed.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

</tbody>

</table>