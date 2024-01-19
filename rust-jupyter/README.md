# Rust x Jupyter

Integrating Rust into Jupyter notebooks with EvCxR (Evaluation Context for Rust)


## Install evcxr-jupyter

```
# NB: .devcontainer/Dockerfile must include `jupyter-notebooks`
# NB: .devcontainer/devcontainer.json must include jupyter vscode ext `ms-toolsai.jupyter`

cd rust-jupyter
make toolchain
make install
```

## Run Notebook

1. Launch `./notebook.ipynb` >> Select Kernel >> Jupyter Kernel >> Rust
2. Run All Cells

## Resources

* [EvCxR docs](https://github.com/evcxr/evcxr/tree/main/evcxr_jupyter)
* [EvCxR Jupyter Demo](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb) 


