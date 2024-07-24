# TODOs

- elevation
- shadow
- border radius (if use component var it is not work) => use 20px border radius



# Reference

- [Common Button -> Specs](https://m3.material.io/components/buttons/specs)
- [Filled tonal button](https://m3.material.io/components/buttons/specs#158f0a18-67fb-4ac4-9d22-cc4d1adc4579)
- [Default values (enabled state)](https://m3.material.io/components/buttons/specs#d6ce6169-536d-4da1-b2fe-91c14de1d015)
- [Filled tonal button states](https://m3.material.io/components/buttons/specs#f1ee3b3c-c575-4a27-8503-1769fb5ce4de)
- [Filled tonal button measurements](https://m3.material.io/components/buttons/specs#a256e1d3-8aa0-4c66-84d5-aebc113714e9)
- [baseline css](../../tokens/css/baseline.css)
- [component css](../../tokens/css/components/filled-tonal-button.css)




<table>

<thead>

<tr>

<th>State</th>

<th>Element</th>

<th>Design attribute</th>

<th>Role</th>

<th>Token or value</th>

</tr>

</thead>

<tbody>

<tr>

<th rowspan="10">1\. Enabled  
</th>

<td>Container</td>

<td>Color</td>

<td>Secondary container</td>

<td>[md.sys.color.secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td> </td>

<td>Shadow color</td>

<td>Shadow</td>

<td>[md.sys.color.shadow](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On secondary</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="6">2\. Disabled  
</th>

<td>Container</td>

<td>Color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td> </td>

<td>Opacity</td>

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

<td> </td>

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

<td> </td>

<td>Opacity</td>

<td>-</td>

<td>38%</td>

</tr>

<tr>

<th rowspan="5">3\. Hovered  
</th>

<td>Container</td>

<td>State layer color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td> </td>

<td>State layer opacity</td>

<td>Hover state layer opacity</td>

<td>[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td> </td>

<td>Elevation</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="5">4\. Focused  
</th>

<td>Container</td>

<td>State layer color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td> </td>

<td>State layer opacity</td>

<td>Focus state layer opacity</td>

<td>[md.sys.state.focus.state-layer-opacity](/m3/pages/interaction-states)</td>

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

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="5">5\. Pressed  
</th>

<td>Container</td>

<td>State layer color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td> </td>

<td>State layer opacity</td>

<td>Pressed state layer opacity</td>

<td>[md.sys.state.pressed.state-layer-opacity](/m3/pages/interaction-states)</td>

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

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

</tbody>

</table>