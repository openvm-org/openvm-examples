with open('outputs/input_data2.txt', 'wb') as f:    # Note the 'wb' mode for binary writing
    n = 10
    f.write(n.to_bytes(8, "little"))
