```shell
$ python3 -m venv /tmp/venv
$ source /tmp/venv/bin/activate
$ pip install numpy scipy maturin
$ maturin develop --release --strip
$ ./fit.py
python result:   message: Desired error not necessarily achieved due to precision loss.
  success: False
   status: 2
      fun: 1.1031200477455258e-10
        x: [ 1.000e-01  5.000e-01]
      nit: 5
      jac: [ 6.388e-05  3.162e-05]
 hess_inv: [[ 5.000e-07 -1.957e-13]
            [-1.957e-13  5.000e-07]]
     nfev: 54
     njev: 14
average python time taken: 2.8219323635101317
rust result:   message: Desired error not necessarily achieved due to precision loss.
  success: False
   status: 2
      fun: 1.1084235720299592e-10
        x: [ 1.000e-01  5.000e-01]
      nit: 6
      jac: [ 1.644e-04 -1.387e-04]
 hess_inv: [[ 5.000e-07 -8.705e-13]
            [-8.705e-13  5.000e-07]]
     nfev: 72
     njev: 20
average rust time taken (parallel=False): 2.5835437774658203
rust result:   message: Optimization terminated successfully.
  success: True
   status: 0
      fun: 1.1102226029039719e-10
        x: [ 1.000e-01  5.000e-01]
      nit: 6
      jac: [-1.754e-12 -2.802e-11]
 hess_inv: [[ 5.000e-07 -1.936e-13]
            [-1.936e-13  5.000e-07]]
     nfev: 33
     njev: 11
average rust time taken (parallel=True): 0.1505760669708252
```
