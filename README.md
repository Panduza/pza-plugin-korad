# pza-plugin-korad





```json

{
    "identity" // IDN string

    "control" { // class
        "output_enable" // OUT (boolean) control

        "voltage": {// class - tag SI
            "value" // VSET
            "unit"  // String "V"
        },
        "current": {// class - tag SI
            "value" // ISET
            "unit"  // String "A"
        },

        "options": {// class
            "ocp" // OCP (boolean) control
            "ovp" // OVP (boolean) control
            "beep" // BEEP (boolean)
            "Lock" // status
            "mode" { // class enum string
                "choices" // ["C.C"] ["C.V"]
                "value"  // string
            },
            "Tracking" // ????  there is a ref in the doc but...
        }
    },
    "measure" { // class
        "voltmeter" { // class - tag SI
            "value" // VOUT
            "unit"  // String "V"
        },
        "ampermeter" {// class - tag SI
            "value" // IOUT
            "unit"  // String "A"
        }
    },



    // I'll look on those parameters later
    "RCL" // 
    "SAV" // 

}


```

