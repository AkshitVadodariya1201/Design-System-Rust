# TODOs

- elevation 
- shadow color
- Surface container high & Surface container low  => used --ad-ref-palette-neutral20
- small FAB, large FAB, medium FAB => hight, width, icon size, border radius not defined
- tertiary color light & dark => palette changed
- Keyboard navigation
  - Tab  => 	Focus lands on FAB
  - Space / enter  =>	Perform the default action on an item
  - Swipe left / right =>	Go up & down the menu options (Android and iOS)



# References

- [Floating action buttons (FAB) => Specs](https://m3.material.io/components/floating-action-button/specs)

### FAB
- [FAB](https://m3.material.io/components/floating-action-button/specs#71504201-7bd1-423d-8bb7-07e0291743e5)
- [FAB => Default values (enabled state)](https://m3.material.io/components/floating-action-button/specs#ea1d22d1-0751-405d-8743-99e36792f245)
- [Additional color mappings](https://m3.material.io/components/floating-action-button/specs#533edd36-9244-456e-96a6-769290df5c61)
- [FAB states](https://m3.material.io/components/floating-action-button/specs#9a5384e5-87ac-4a9a-8877-d51a46bc0c74)
- [FAB measurements](https://m3.material.io/components/floating-action-button/specs#b54e2a00-0822-4f0d-8d9d-aba65cb8f856)

### Small FAB
- [Small FAB](https://m3.material.io/components/floating-action-button/specs#df918e03-5939-4aa4-8d4b-4cdffa52b240)
- [Small FAB => Default values (enabled state)](https://m3.material.io/components/floating-action-button/specs#5addee58-138a-4931-9084-1723ad584b04)
- [Additional color mappings](https://m3.material.io/components/floating-action-button/specs#618c0863-b021-4a49-a65f-50799cb22fec)
- [Small FAB states](https://m3.material.io/components/floating-action-button/specs#1db5bb2b-5a92-4988-bd5c-3f29bd8ab081)
- [Small FAB measurements](https://m3.material.io/components/floating-action-button/specs#aa9b19fa-53a4-4749-90d1-5a05035d5c08)


### Large FAB
- [Large FAB](https://m3.material.io/components/floating-action-button/specs#9d7d3d6a-bab7-47cb-be32-5596fbd660fe)
- [Large FAB => Default values (enabled state)](https://m3.material.io/components/floating-action-button/specs#d4a159ce-8fb6-4262-8b7d-aa2111168181)
- [Additional color mappings](https://m3.material.io/components/floating-action-button/specs#be4d7186-ff7d-4f20-b7cc-70f3d83c4d6a)
- [Large FAB states](https://m3.material.io/components/floating-action-button/specs#f17c9cfc-c81d-408a-b020-c27f9e4f438a)
- [Large FAB measurements](https://m3.material.io/components/floating-action-button/specs#90b60c40-53b8-43ee-af63-583f9f1dac98)
- [baseline css](../../tokens/css/baseline.css)
- [component css](../../tokens/css/components/fab.css)



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

<th rowspan="5">1\. Enabled  
</th>

<td>Container</td>

<td>Color</td>

<td>Surface</td>

<td>[md.sys.color.surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td> </td>

<td>Shadow color</td>

<td>Shadow</td>

<td>[md.sys.color.shadow](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td> </td>

<td>Elevation (default)</td>

<td>Level 3</td>

<td>[md.sys.elevation.level3](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td> </td>

<td>Elevation (lowered)</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

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

<td>Container</td>

<td>Elevation (default)</td>

<td>Level 4</td>

<td>[md.sys.elevation.level4](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td> </td>

<td>Elevation (lowered)</td>

<td>Level 2</td>

<td>[md.sys.elevation.level2](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td> </td>

<td>State layer color</td>

<td>On primary container</td>

<td>[md.sys.color.on-primary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)*</td>

</tr>

<tr>

<td> </td>

<td>State layer opacity</td>

<td>Hover state layer opacity</td>

<td>[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<th rowspan="4">3\. Focused  
</th>

<td>Container</td>

<td>Elevation (default)</td>

<td>Level 3</td>

<td>[md.sys.elevation.level3](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td> </td>

<td>Elevation (lowered)</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td> </td>

<td>State layer color</td>

<td>On primary container</td>

<td>[md.sys.color.on-primary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)*</td>

</tr>

<tr>

<td> </td>

<td>State layer opacity</td>

<td>Focus state layer opacity</td>

<td>[md.sys.state.focus.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

<tr>

<th rowspan="4">4\. Pressed  
</th>

<td>Container</td>

<td>Elevation (default)</td>

<td>Level 3</td>

<td>[md.sys.elevation.level3](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td> </td>

<td>Elevation (lowered)</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td> </td>

<td>State layer color</td>

<td>On primary container</td>

<td>[md.sys.color.on-primary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)*</td>

</tr>

<tr>

<td> </td>

<td>State layer opacity</td>

<td>Pressed state layer opacity</td>

<td>[md.sys.state.pressed.state-layer-opacity](/m3/pages/interaction-states)</td>

</tr>

</tbody>

</table>