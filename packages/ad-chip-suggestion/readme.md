# TODOS


- elevation
- shadow color
- dragged
- container background color -> (surface container low ) not define  => use container background color -> var(--ad-ref-palette-neutral95)
- Keyboard navigation
  - Tab =>	Focus lands on (non-disabled) chip
  - Space or Enter =>	Focus lands on (non-disabled) chip
  - Backspace or Delete =>	Removes current focused input chip


# Reference

- [Chips => Specs](https://m3.material.io/components/chips/specs)
- [Suggestion chip](https://m3.material.io/components/chips/specs#67a358c0-c370-4bf1-b410-7f8dd3f1a60c)
- [Suggestion chip => Default values (enabled state)](https://m3.material.io/components/chips/specs#09636b0e-94aa-454d-81e3-9ba83b8839cf)
- [Suggestion chip states](https://m3.material.io/components/chips/specs#05fa69e8-8a9a-4a6e-b011-8dedce4b2637)
- [Suggestion chip measurements](https://m3.material.io/components/chips/specs#b6d645e0-5632-4667-9ef5-8d920adc6f1d)

- [baseline css](../../tokens/css/baseline.css)
- [components css](../../tokens/css/components/suggestion-chip.css)





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

<th>Token  
</th>

</tr>

</thead>

<tbody>

<tr>

<th rowspan="11">1\. Enabled  
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

<th rowspan="7">2\. Disabled  
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

<td>Container  
(elevated style)</td>

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

<th rowspan="4">3\. Hovered  
</th>

<td>Container: elevated</td>

<td>Elevation</td>

<td>Level 2</td>

<td>[md.sys.elevation.level2](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer</td>

<td>State layer color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>state layer opacity</td>

<td>Hover state layer opacity</td>

<td>md.sys.state.hover.state-layer-opacity</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="5">4\. Focused  
</th>

<td>Container</td>

<td>Outline color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>state layer opacity</td>

<td>Focus state layer opacity</td>

<td>md.sys.state.focus.state-layer-opacity</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="4">5\. Pressed  
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

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>state layer opacity</td>

<td>Pressed state layer opacity</td>

<td>md.sys.state.pressed.state-layer-opacity</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="4">6\. Dragged  
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

<td>state layer opacity</td>

<td>Dragged state layer opacity</td>

<td>md.sys.state.dragged.state-layer-opacity</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

</tbody>

</table>