# TODOs

- elevation
- shadow color
- component token some properties are wrong
- Surface container high & Surface container low  => used --ad-ref-palette-neutral20
- Keyboard navigation
  - Tabs  =>	Focus lands on FAB
  - Space or Enter  =>	Perform the default action on an item
  - Swipe or Arrow  =>	Move up and down the navigation options



# References

- [Extended FAB => Specs](https://m3.material.io/components/extended-fab/specs)
- [Extended FAB => Default values (enabled state)](https://m3.material.io/components/extended-fab/specs#686d689a-4d88-4f01-9d2e-8d94f5ac3c02)
- [Additional color mappings](https://m3.material.io/components/extended-fab/specs#b2f6f3d2-b061-4fbe-8234-75d7c0d1ce09)
- [Extended FAB states](https://m3.material.io/components/extended-fab/specs#99869d8c-3010-4519-97e3-ac50540ce34e)
- [Extended FAB measurements](https://m3.material.io/components/extended-fab/specs#2f7813c6-6f0b-4301-bdcc-4243f63dfb3f)
[component css](../../tokens/css/components/extended-fab.css)
[baseline css](../../tokens/css/baseline.css)



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

<td rowspan="4">Container  
</td>

<td>Color</td>

<td>Primary container</td>

<td>[md.sys.color.primary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Shadow color</td>

<td>Shadow</td>

<td>[md.sys.color.shadow](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Elevation (default)</td>

<td>Level 3</td>

<td>[md.sys.elevation.level3](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Elevation (lowered)</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td rowspan="6">Label text  
</td>

<td>Color</td>

<td>On primary container</td>

<td>[md.sys.color.on-primary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Font</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.font](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td>Line height</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.line-height](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td>Size</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.size](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td>Tracking</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.tracking](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td>Weight</td>

<td>Label large</td>

<td>[md.sys.typescale.label-large.weight](/m3/pages/typography/tokens#d74b73c2-ac5d-43c5-93b3-088a2f67723d)</td>

</tr>

<tr>

<td>Icon</td>

<td>Color</td>

<td>On primary container</td>

<td>[md.sys.color.on-primary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="4">2\. Hovered  
</th>

<td rowspan="4">Container  
</td>

<td>Elevation (default)</td>

<td>Level 4</td>

<td>[md.sys.elevation.level4](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Elevation (lowered)</td>

<td>Level 2</td>

<td>[md.sys.elevation.level2](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer color</td>

<td>On primary container</td>

<td>[md.sys.color.on-primary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)*</td>

</tr>

<tr>

<td>State layer opacity</td>

<td>Hover state layer opacity</td>

<td>[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<th rowspan="4">3\. Focused  
</th>

<td rowspan="4">Container  
</td>

<td>Elevation (default)</td>

<td>Level 3</td>

<td>[md.sys.elevation.level3](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Elevation (lowered)</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer color</td>

<td>On primary container</td>

<td>[md.sys.color.on-primary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)*</td>

</tr>

<tr>

<td>State layer opacity</td>

<td>Hover state layer opacity</td>

<td>[md.sys.state.focus.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<th rowspan="4">4\. Pressed  
</th>

<td rowspan="4">Container  
</td>

<td>Elevation (default)</td>

<td>Level 3</td>

<td>[md.sys.elevation.level3](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Elevation (lowered)</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>State layer color</td>

<td>On primary container</td>

<td>[md.sys.color.on-primary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)*</td>

</tr>

<tr>

<td>State layer opacity</td>

<td>Pressed state layer opacity</td>

<td>[md.sys.state.press.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

</tbody>

</table>