# TODOs

- toggle event
- Surface container highest not defined => use --ad-ref-palette-neutral20
- Keyboard navigation 
    1. Space/Enter: Activates the (non-disabled) button
    2. Tab: Focuses the button


# References

- [Icon buttons => Specs](https://m3.material.io/components/icon-buttons/specs)
- [Filled icon button](https://m3.material.io/components/icon-buttons/specs#d4169fb5-4cf8-40b6-9ec3-4044f09cca1f)
- [Filled icon button => Default values (enabled state)](https://m3.material.io/components/icon-buttons/specs#fbe6913b-2c1b-42c1-be4a-3d73d9dc601d)
- [Filled icon button states](https://m3.material.io/components/icon-buttons/specs#e1b5554a-eef9-4b41-a598-a40bffb31a7a)
- [Filled icon button measurements](https://m3.material.io/components/icon-buttons/specs#545cdffc-4b0f-4f4e-ae55-98031586b209)
- [baseline css](../../tokens/css/baseline.css)
- [component css](../../tokens/css/components/filled-icon-button.css)


<table>

<thead>

<tr>

<th>State</th>

<th>Element</th>

<th>Design attribute</th>

<th>Role</th>

<th>Value</th>

</tr>

</thead>

<tbody>

<tr>

<th rowspan="9">1\. Enabled  
</th>

<td>Icon</td>

<td>Size</td>

<td>-</td>

<td>24dp</td>

</tr>

<tr>

<td></td>

<td>Color (no toggle - default)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (toggle-unselected)</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (toggle -selected)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Container</td>

<td>Size</td>

<td>-</td>

<td>40dp</td>

</tr>

<tr>

<td></td>

<td>Shape</td>

<td>Full</td>

<td>[md.sys.shape.corner.full](/m3/pages/shape/shape-scale-tokens#6f668ba1-b671-4ea2-bcf3-c1cff4f4099e)</td>

</tr>

<tr>

<td></td>

<td>Color (no toggle - default)</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>Surface container highest</td>

<td>[md.sys.color.surface-container-highest](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>Color</td>

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

<th rowspan="7">3\. Hovered  
</th>

<td>State layer</td>

<td>Opacity</td>

<td>Hover state layer opacity</td>

<td>md.sys.state.hover.state-layer-opacity</td>

</tr>

<tr>

<td></td>

<td>Color (no toggle - default)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon</td>

<td>Color (no toggle-defafult)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="7">4\. Focused  
</th>

<td>State layer</td>

<td>Opacity</td>

<td>Focus state layer opacity</td>

<td>md.sys.state.focus.state-layer-opacity</td>

</tr>

<tr>

<td></td>

<td>Color (no toggle- default)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon</td>

<td>Color (no toggle- default)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<th rowspan="7">5\. Pressed  
</th>

<td>State layer</td>

<td>Opacity</td>

<td>Pressed state layer opacity</td>

<td>md.sys.state.pressed.state-layer-opacity</td>

</tr>

<tr>

<td></td>

<td>Color(no toggle - default)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color(unselected)</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon</td>

<td>Color (no toggle - default)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>Primary</td>

<td>[md.sys.color.primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On primary</td>

<td>[md.sys.color.on-primary](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

</tbody>

</table>