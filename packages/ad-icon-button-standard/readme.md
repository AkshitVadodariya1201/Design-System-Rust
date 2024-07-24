# TODOs

- toggle button
- standard icon button component css not found => use outlined icon button css
- keyboard navigation
    1. Space/Enter: Activates the (non-disabled) button
    2. Tab: Focuses the button


# References

- [Icon buttons => Specs](https://m3.material.io/components/icon-buttons/specs)
- [Standard icon button](https://m3.material.io/components/icon-buttons/specs#eca0451e-430b-41e1-bea3-a31cb7ccda76)
- [Standard  icon button => Default values (enabled state)](https://m3.material.io/components/icon-buttons/specs#b5e4c0a5-c9cf-413b-9f26-8ee2e899c41b)
- [Standard icon button states](https://m3.material.io/components/icon-buttons/specs#4ccca1dd-ce17-4bb7-bce5-fb709923a193)
- [Standard icon button measurements](https://m3.material.io/components/icon-buttons/specs#407b5bca-992c-42ec-bc96-c78935ddba0e)
- [baseline css](../../tokens/css/baseline.css)
- [component css](../../tokens/css/components/icon-button.css)


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

<th rowspan="5">1\. Enabled  
</th>

<td>Icon</td>

<td>Size</td>

<td>-</td>

<td>24dp</td>

</tr>

<tr>

<td>Icon</td>

<td>Color (unselected)</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>State layer</td>

<td>Size</td>

<td>-</td>

<td>40dp</td>

</tr>

<tr>

<td>State layer</td>

<td>Shape</td>

<td>Full</td>

<td>[md.sys.shape.corner.full](/m3/pages/shape/shape-scale-tokens#6f668ba1-b671-4ea2-bcf3-c1cff4f4099e)</td>

</tr>

<tr>

<th rowspan="2">2\. Disabled  
</th>

<td>Icon</td>

<td>Color</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>-%</td>

<td>38%</td>

</tr>

<tr>

<th rowspan="6">3\. Hovered  
</th>

<td>State layer (unselected)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td></td>

<td>State layer opacity</td>

<td>md.sys.state.hover.state-layer-opacity</td>

</tr>

<tr>

<td>Icon (unselected)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>State layer (selected)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td></td>

<td>State layer opacity</td>

<td>md.sys.state.hover.state-layer-opacity</td>

</tr>

<tr>

<td>Icon (selected)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="6">4\. Focused  
</th>

<td>State layer (unselected)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td></td>

<td>State layer opacity</td>

<td>md.sys.state.hover.state-layer-opacity</td>

</tr>

<tr>

<td>Icon (unselected)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>State layer (selected)</td>

<td>Color</td>

<td></td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td></td>

<td>State layer opacity</td>

<td>md.sys.state.focus.state-layer-opacity</td>

</tr>

<tr>

<td>Icon (selected)</td>

<td>Color</td>

<td></td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="6">5\. Pressed  
</th>

<td>State layer (unselected)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td></td>

<td>State layer opacity</td>

<td>md.sys.state.pressed.state-layer-opacity</td>

</tr>

<tr>

<td>Icon (unselected)</td>

<td>Color</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>State layer (selected)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td></td>

<td>State layer opacity</td>

<td>md.sys.state.pressed.state-layer-opacity</td>

</tr>

<tr>

<td>Icon (selected)</td>

<td>Color</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

</tbody>

</table>