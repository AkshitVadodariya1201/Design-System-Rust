# TODOs


- border radius not work properly
- background-color: var(--ad-sys-color-surface-container-low); not found => use --ad-ref-palette-neutral95
- elevation
- shadow color
- Keyboard navigation



# Reference


- [Common Button -> Specs](https://m3.material.io/components/buttons/specs)
- [Elevated button](https://m3.material.io/components/buttons/specs#0eea2a85-b4d7-4c74-b08e-98410b9412c7)
- [Default values (enabled state)](https://m3.material.io/components/buttons/specs#7026d152-e4c1-4de9-9d06-5631c09c3f4b)
- [Elevated button states](https://m3.material.io/components/buttons/specs#c46608e1-3820-4da5-b1ec-b913992852f1)
- [Elevated button measurements](https://m3.material.io/components/buttons/specs#85e63496-f905-4978-ae35-69ab83b70536)

- [baseline css](../../tokens/css/baseline.css)
- [component css](../../tokens/css/components/elevated-button.css)




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

<th rowspan="10">1\. Enabled  
</th>

<td>Container</td>

<td>Color</td>

<td>Surface container low</td>

<td>[md.sys.color.surface-container-low](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Shadow color</td>

<td>Shadow</td>

<td>[md.sys.color.shadow](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Elevation</td>

<td>Level  1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>Label text</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>Icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="7">2\. Disabled  
</th>

<td>Container</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens?preview=true#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>Container</td>

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

<td></td>

<td>Elevation</td>

<td>Level 2</td>

<td>[md.sys.elevation.level2](/m3/pages/elevation/tokens)</td>

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

<th rowspan="5">4\. Focused  
</th>

<td>Container</td>

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

<td></td>

<td>Elevation</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

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

<th rowspan="5">5\. Pressed  
</th>

<td>Container</td>

<td>State layer color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>State layer opacity</td>

<td>Pressed state layer opacity</td>

<td>[md.sys.state.pressed.state-layer-opacity](/m3/pages/interaction-states/)</td>

</tr>

<tr>

<td></td>

<td>Elevation</td>

<td>Level 1</td>

<td>[md.sys.elevation.level1](/m3/pages/elevation/tokens)</td>

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