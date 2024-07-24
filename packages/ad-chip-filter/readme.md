# TODOs

- elevation
- container background color -> (surface container low) not define   => use background color -> (--ad-ref-palette-neutral95)
- in unselected state, in hover state, label-text color not define ( On surface variant state content )
and used ( On surface variant ) color
- removable icon focus outline height remaining
- dragged
- Keyboard navigation
  - Tab =>	Focus lands on (non-disabled) chip
  - Space or Enter =>	Focus lands on (non-disabled) chip
  - Backspace or Delete =>	Removes current focused input chip


# Reference

- [Chips => Specs](https://m3.material.io/components/chips/specs)
- [Filter chip](https://m3.material.io/components/chips/specs#e900592f-75a4-4298-853c-bedd8f462f83)
- [Filter chip => Default values (enabled state)](https://m3.material.io/components/chips/specs#c66fab5e-824d-4b6e-b9bc-efc8611d090e)
- [Filter chip states](https://m3.material.io/components/chips/specs#546770a4-8710-4dbe-8c36-62fc5bb32e2d)
- [Filter chip measurements](https://m3.material.io/components/chips/specs#590903f7-2bf5-46ab-9810-d052173f41f1)

- [baseline css](../../tokens/css/baseline.css)
- [components css](../../tokens/css/components/filter-chip.css)

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

<th rowspan="19">1\. Enabled  
</th>

<td>Container, unselected</td>

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

<td>Container, unselected (elevated style)</td>

<td>Color</td>

<td>Surface conter low</td>

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

<td>Container, selected</td>

<td>Color</td>

<td>Secondary container</td>

<td>[md.sys.color.secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Outline width</td>

<td>-</td>

<td>0dp</td>

</tr>

<tr>

<td>Container, selected  
(elevated style)</td>

<td>Color</td>

<td>Secondary container</td>

<td>[md.sys.color.secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Elevation</td>

<td>Level1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Label text, unselected</td>

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

<td>Label text, selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Leading icon, unselected (optional)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Leading icon, selected (optional)</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="11">2\. Disabled  
</th>

<td>Container, unselected</td>

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

<td>Container, selected</td>

<td>Container color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Container opacity</td>

<td>-</td>

<td>12%</td>

</tr>

<tr>

<td>Container  
(elevated style)</td>

<td>Color</td>

<td>Level 0</td>

<td>[md.sys.elevation.level0](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td></td>

<td>Elevation</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<th rowspan="11">3\. Hovered  
</th>

<td>Container, unselected</td>

<td>Elevation</td>

<td>Level 0</td>

<td>[md.sys.elevation.level0](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Container, selected</td>

<td>Elevation</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Container  
(elevated style)</td>

<td>Elevation</td>

<td>Level 2</td>

<td>[md.sys.elevation.level2](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer, unselected</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>Hover state layer opacity</td>

<td>[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>State layer selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>Hover state layer opacity</td>

<td>[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text, unselected</td>

<td>Color</td>

<td>On surface variant state content</td>

<td>[md.sys.color.on-surface-variant-state-content](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Label text, selected</td>

<td>Color</td>

<td>Secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional), unselected</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional), selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="11">4\. Focused  
</th>

<td>Container</td>

<td>Elevation</td>

<td>Level 0</td>

<td>[md.sys.elevation.level0](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td></td>

<td>Outline color</td>

<td>On surface variant state content</td>

<td>[md.sys.color.on-surface-variant-state-content](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Container  
(elevated style)</td>

<td>Elevation</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer, unselected</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>Focus state layer opacity</td>

<td>[md.sys.state.focus.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>State layer selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>Focus state layer opacity</td>

<td>[md.sys.state.focus.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text, unselected</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Label text, selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional), unselected</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional), selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="10">5\. Pressed  
</th>

<td>Container</td>

<td>Elevation</td>

<td>Level 0</td>

<td>[md.sys.elevation.level0](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Container  
(elevated style)</td>

<td>Elevation</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer, unselected</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>Pressed state layer opacity</td>

<td>[md.sys.state.pressed.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>State layer selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>Pressed state layer opacity</td>

<td>[md.sys.state.pressed.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text, unselected</td>

<td>Color</td>

<td>On surfacevariant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Label text, selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional), unselected</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional), selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="9">6\. Dragged  
</th>

<td>Container  
(elevated style)</td>

<td>Elevation</td>

<td>Level 4</td>

<td>[md.sys.elevation.level4](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer, unselected</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>Dragged  state layer opacity</td>

<td>[md.sys.state.dragged.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>State layer selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>Dragged  state layer opacity</td>

<td>[md.sys.state.dragged.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<td>Label text, unselected</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Label text, selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional), unselected</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon (optional), selected</td>

<td>Color</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

</tbody>

</table>