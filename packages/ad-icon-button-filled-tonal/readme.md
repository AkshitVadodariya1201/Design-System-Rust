# TODOs

- toggle event
- Surface container highest not defined => use --ad-ref-palette-neutral20
- Keyboard navigation 
    1. Space/Enter: Activates the (non-disabled) button
    2. Tab: Focuses the button


# References

- [Icon buttons => Specs](https://m3.material.io/components/icon-buttons/specs)
- [Filled tonal icon button](https://m3.material.io/components/icon-buttons/specs#c2ca424b-2ad7-40e6-8946-47fb1918060a)
- [Filled tonal icon button => Default values (enabled state)](https://m3.material.io/components/icon-buttons/specs#619fb7b8-afe6-4b3c-9da5-767ca4a332b5)
- [Filled tonal icon button states](https://m3.material.io/components/icon-buttons/specs#9a8c9928-d5dc-4363-9658-4a30dcbca6a3)
- [Filled tonal icon button measurements](https://m3.material.io/components/icon-buttons/specs#2ebe7c40-9a1f-40ed-9f36-427c665f7b8d)
- [baseline css](../../tokens/css/baseline.css)
- [component css](../../tokens/css/components/filled-tonal-icon-button.css)


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

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (toggle-unselected)</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (toggle -selected)</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>Secondary container</td>

<td>[md.sys.color.secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>Secondary container</td>

<td>[md.sys.color.secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>Color (no toggle - default)</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Opacity</td>

<td>Hover state layer opacity</td>

<td>md.sys.state.hover.state-layer-opacity</td>

</tr>

<tr>

<td>Icon</td>

<td>Color (no toggle-defafult)</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon</td>

<td>Color (no toggle- default)</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

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

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color(unselected)</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (selected)</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td>Icon</td>

<td>Color (no toggle - default)</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td>Color (unselected)</td>

<td>On surface variant</td>

<td>[md.sys.color.on-surface-variant](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

<tr>

<td></td>

<td> Color (selected)</td>

<td>On secondary container</td>

<td>[md.sys.color.on-secondary-container](/m3/pages/using-color/tokens#7fd4440e-986d-443f-8b3a-4933bff16646)</td>

</tr>

</tbody>

</table>