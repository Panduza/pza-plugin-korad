# pza-plugin-korad

```
cargo post build --features plugin
```



## KD3005P_fake

This driver provides a very simulated version of the KD3005P.

```json
{
    "devices": [
        {
            "name": "my_power_supply",
            "dref": "korad.KD3005P_fake"
        }
    ]
}
```

For now only, the only attribute implemented are:

- control/output_enable : start at false, then just memorize the last set value
- control/voltage : start at 0, then just memorize the last set value
- control/current : start at 0, then just memorize the last set value
