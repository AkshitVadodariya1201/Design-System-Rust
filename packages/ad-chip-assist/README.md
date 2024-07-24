# TODOs

- active , focus , hover -> elevated container - elevation
- dragged 
- container background color -> (surface container low) not define   => use container background color -> var(--ad-ref-palette-neutral95);
- Keyboard navigation
  - Tab =>	Focus lands on (non-disabled) chip
  - Space or Enter =>	Focus lands on (non-disabled) chip
  - Backspace or Delete =>	Removes current focused input chip



# Reference

- [Chips => Specs](https://m3.material.io/components/chips/specs)
- [Assist chip](https://m3.material.io/components/chips/specs#a144389c-9478-4fe4-9bd8-ca9f7dd830eb)
- [Assist chip => Default values (enabled state)](https://m3.material.io/components/chips/specs#e8514405-bc24-4cea-851a-e910a17db29c)
- [Assist chip states](https://m3.material.io/components/chips/specs#08af13fa-c634-44bc-a19b-99e730aed4ee)
- [Assist chip measurements](https://m3.material.io/components/chips/specs#b64e38bb-43c7-4b44-b51c-4100c6344a8d)

- [baseline css](../../tokens/css/baseline.css)
- [components css](../../tokens/css/components/assist-chip.css)




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

<th rowspan="14">1\. Enabled  
</th>

<td>Container</td>

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

<td>Container  
(elevated style)</td>

<td>Color</td>

<td>Surface container low</td>

<td>[md.sys.color.surface-container-low](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Elevation</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td></td>

<td>Shadow color</td>

<td>Shadow</td>

<td>[md.sys.color.shadow](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Trailing icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>Container (elevated style)</td>

<td>Color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Elevation</td>

<td>Level 0</td>

<td>[md.sys.elevation.level0](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td></td>

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

<th rowspan="5">3\. Hovered  
</th>

<td>Container  
(elevated style)</td>

<td>Elevation</td>

<td>Level 2</td>

<td>[md.sys.elevation.level2](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer</td>

<td>State layer color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>state layer opacity</td>

<td>Hover state layer opacity</td>

<td>[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="6">4\. Focused  
</th>

<td>Container</td>

<td>Outline color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Container  
(elevated style)</td>

<td>Elevation</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer</td>

<td>State layer color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>state layer opacity</td>

<td>Focus state layer opacity</td>

<td>[md.sys.state.focus.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="5">5\. Pressed  
</th>

<td>Container  
(elevated style)</td>

<td>Elevation</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer</td>

<td>State layer color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>state layer opacity</td>

<td>Pressed state layer opacity</td>

<td>[md.sys.state.pressed.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="5">6\. Dragged  
</th>

<td>Container</td>

<td>Elevation</td>

<td>Level 4</td>

<td>[md.sys.elevation.level4](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer</td>

<td>State layer color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>state layer opacity</td>

<td>Dragged state layer opacity</td>

<td>[md.sys.state.dragged.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

</tbody>

</table>