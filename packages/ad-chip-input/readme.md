# TODOs


- dragged
- elevation
- Keyboard navigation
  - Tab =>	Focus lands on (non-disabled) chip
  - Space or Enter =>	Focus lands on (non-disabled) chip
  - Backspace or Delete =>	Removes current focused input chip




# Reference

- [Chips => Specs](https://m3.material.io/components/chips/specs)
- [Input chip](https://m3.material.io/components/chips/specs#facb7c02-74c4-4b81-bd52-6ad10ce351eb)
- [Input chip => Default values (enabled state)](https://m3.material.io/components/chips/specs#ba8ff71c-3d8f-4d25-b690-4d91e96faee1)
- [Input chip states](https://m3.material.io/components/chips/specs#e9b4807e-d38a-42e6-8c2d-608392ca7638)
- [Input chip measurements](https://m3.material.io/components/chips/specs#118ee251-1650-49f6-a384-b7a5f80f13af)

- [baseline css](../../tokens/css/baseline.css)
- [components css](../../tokens/css/components/input-chip.css)



<table>

<thead>

<tr>

<th>State  
</th>

<th>Element  
</th>

<th>Design attribute  
</th>

<th>Role  
</th>

<th>Token or value  
</th>

</tr>

</thead>

<tbody>

<tr>

<th rowspan="13">1\. Enabled  
</th>

<td>Container</td>

<td>Color</td>

<td>Surface</td>

<td>[md.sys.color.surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Elevation</td>

<td>Level 0</td>

<td>[md.sys.elevation.level0](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td></td>

<td>Outline color</td>

<td>Outline</td>

<td>[md.sys.color.outline](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Outline width</td>

<td>-</td>

<td>1dp</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Font</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.font](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td></td>

<td>Line height</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.line-height](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td></td>

<td>Size</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.size](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td></td>

<td>Tracking</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.tracking](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td></td>

<td>Weight</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.weight](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td>Leading icon (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Trailing icon (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Avatar (optional)</td>

<td>Shape</td>

<td>-</td>

<td>12dp</td>

</tr>

<tr>

<th rowspan="9">2\. Disabled  
</th>

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

<td>Trailing icon (optional)</td>

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

<td>Leading icon (optional)</td>

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

<td>Avatar</td>

<td>Opacity</td>

<td>-</td>

<td>38%</td>

</tr>

<tr>

<th rowspan="5">3\. Hovered  
</th>

<td>State layer</td>

<td>State layer color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Trailing icon (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Leading icon (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="6">4\. Focused  
</th>

<td>Container</td>

<td>Outline color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>State layer</td>

<td>State layer color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Trailing icon (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Leading icon (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="5">5\. Pressed  
</th>

<td>State layer</td>

<td>State layer color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Trailing icon (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Leading icon (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="6">6\. Dragged  
</th>

<td>Container</td>

<td>Elevation</td>

<td>Level 4</td>

<td>[md.sys.elevation.level4](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer</td>

<td>State layer color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>State layer opacity</td>

<td>Dragged state layer opacity</td>

<td>[md.sys.state.dragged.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Trailing icon (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Leading icon (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

</tbody>

</table>