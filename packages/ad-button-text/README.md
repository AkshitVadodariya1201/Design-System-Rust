# TODOs

- elevation
- border radius not work 
- Space/Enter: Activates the (non-disabled) button


# References

- [Common Button -> Specs](https://m3.material.io/components/buttons/specs)
- [Text button](https://m3.material.io/components/buttons/specs#899b9107-0127-4a01-8f4c-87f19323a1b4)
- [Default values (enabled state)](https://m3.material.io/components/buttons/specs#a0952e51-6975-486e-8d14-c78283c29cae)
- [Text button states](https://m3.material.io/components/buttons/specs#1e9d216a-3cc8-4a9b-9658-0ae96e1b157c)
- [Text button measurements](https://m3.material.io/components/buttons/specs#05946a72-3805-4949-96cc-0a95359f64c0)
- [baseline css](../../tokens/css/baseline.css)
- [component css](../../tokens/css/components/text-button.css)


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

<th rowspan="8">1\. Enabled  
</th>

<td>1\. Container</td>

<td>Elevation</td>

<td>Level 0</td>

<td>[md.sys.elevation.level0](/m3/pages/elevation/tokens)</td>

</tr>

<tr>

<td>2\. Label text</td>

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

<td>3\. Icon (optional)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="4">2\. Disabled  
</th>

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

<th rowspan="4">3\. Hovered  
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

<td>[md.sys.state.hover.state-layer-opacity](/m3/pages/interaction-states/)</td>

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

<th rowspan="4">4\. Focused  
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

<td>[md.sys.state.focus.state-layer-opacity](/m3/pages/interaction-states/)</td>

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

<th rowspan="4">5\. Pressed  
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