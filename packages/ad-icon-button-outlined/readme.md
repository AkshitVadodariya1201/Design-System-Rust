# TODOs

- toggle button
- Keyboard navigation 
    1. Space/Enter: Activates the (non-disabled) button
    2. Tab: Focuses the button



# References

- [Icon buttons => Specs](https://m3.material.io/components/icon-buttons/specs)
- [Outlined icon button](https://m3.material.io/components/icon-buttons/specs#632e1356-8002-4ae1-ae36-48c1f9b17ef2)
- [Outlined  icon button => Default values (enabled state)](https://m3.material.io/components/icon-buttons/specs#2bd37192-6546-4650-a26a-6cac887e4901)
- [Outlined icon button states](https://m3.material.io/components/icon-buttons/specs#6d6dc8f8-e11c-48e6-8745-21bfcf7ff12c)
- [Outlined icon button measurements](https://m3.material.io/components/icon-buttons/specs#4f171dcf-704f-454e-834f-905fb44acf63)
- [baseline css](../../tokens/css/baseline.css)
- [component css](../../tokens/css/components/outlined-icon-button.css)



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

<th rowspan="8">1\. Enabled  
</th>

<td>Icon</td>

<td>Size</td>

<td>-</td>

<td>24dp</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On inverse surface</td>

<td>[md.sys.color.on-inverse-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Container</td>

<td>Outline color (unselected)</td>

<td>Outline</td>

<td>[md.sys.color.outline](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Outline width (unselected)</td>

<td>-</td>

<td>1dp</td>

</tr>

<tr>

<td></td>

<td>Shape</td>

<td>Full</td>

<td>[md.sys.shape.corner.full](/m3/pages/shape/shape-scale-tokens#6f668ba1-b671-4ea2-bcf3-c1cff4f4099e)</td>

</tr>

<tr>

<td></td>

<td>Size</td>

<td>-</td>

<td>40dp</td>

</tr>

<tr>

<td></td>

<td>Color (selected only)</td>

<td>Inverse surface</td>

<td>[md.sys.color.inverse-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="4">2\. Disabled  
</th>

<td>Icon</td>

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

<th rowspan="5">3\. Hovered  
</th>

<td>State layer</td>

<td>Opacity</td>

<td>Hover state layer opacity</td>

<td>md.sys.state.hover.state-layer-opacity</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On inverse surface</td>

<td>[md.sys.color.on-inverse-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On inverse surface</td>

<td>[md.sys.color.on-inverse-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="5">4\. Focused  
</th>

<td>State layer</td>

<td>Opacity</td>

<td>Focus state layer opacity</td>

<td>md.sys.state.focus.state-layer-opacity</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On inverse surface</td>

<td>[md.sys.color.on-inverse-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On inverse surface</td>

<td>[md.sys.color.on-inverse-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="5">5\. Pressed  
</th>

<td>State layer</td>

<td>Opacity</td>

<td>Pressed state layer opacity</td>

<td>md.sys.state.pressed.state-layer-opacity</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On inverse surface</td>

<td>[md.sys.color.on-inverse-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon</td>

<td>Color (unselected)</td>

<td>On surface</td>

<td>[md.sys.color.on-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On inverse surface</td>

<td>[md.sys.color.on-inverse-surface](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

</tbody>

</table>