---

---

# Model Card for Census Income Classifier

<!-- Provide a quick summary of what the model is/does. [Optional] -->
This is a wide and deep Keras model which aims to classify whether or not an individual has an income of over $50,000 based on various demopraphic features. The model is trained on the UCI Census Income Dataset. This is not a production model, and this dataset has traditionally only been used for research purposes. In this Model Card, you can review quantative components of the nmodel's performance and data, as well as information about the model's intended uses, limitations, and ethical considerations.

## Table of Contents

- [Model Details](#model-details)
  - [Model Description](#model-description)
- [Uses](#uses)
  - [Direct Use](#direct-use)
  - [Downstream Use [Optional]](#downstream-use-optional)
  - [Out-of-Scope Use](#out-of-scope-use)
- [Bias, Risks, and Limitations](#bias-risks-and-limitations)
  - [Recommendations](#recommendations)
- [Training Details](#training-details)
  - [Training Data](#training-data)
  - [Training Procedure](#training-procedure)
    - [Preprocessing](#preprocessing)
    - [Speeds, Sizes, Times](#speeds-sizes-times)
- [Evaluation](#evaluation)
  - [Testing Data, Factors & Metrics](#testing-data-factors--metrics)
    - [Testing Data](#testing-data)
    - [Factors](#factors)
    - [Metrics](#metrics)
  - [Results](#results)
- [Model Examination](#model-examination)
- [Environmental Impact](#environmental-impact)
- [Technical Specifications [optional]](#technical-specifications-optional)
  - [Model Architecture and Objective](#model-architecture-and-objective)
  - [Compute Infrastructure](#compute-infrastructure)
    - [Hardware](#hardware)
    - [Software](#software)
- [Citation](#citation)
- [Glossary [optional]](#glossary-optional)
- [More Information [optional]](#more-information-optional)
- [Model Card Authors [optional]](#model-card-authors-optional)
- [Model Card Contact](#model-card-contact)
- [How to Get Started with the Model](#how-to-get-started-with-the-model)

## Model Details

### Model Description

<!-- Provide a longer summary of what this model is/does. -->
This is a wide and deep Keras model which aims to classify whether or not an individual has an income of over $50,000 based on various demopraphic features. The model is trained on the UCI Census Income Dataset. This is not a production model, and this dataset has traditionally only been used for research purposes. In this Model Card, you can review quantative components of the nmodel's performance and data, as well as information about the model's intended uses, limitations, and ethical considerations.

- **Developed by:** Daniel Vorhauer <daniel.vorhauer@company.com>
- **Shared by [Optional]:** Data Science Team <datascience@company.com>
- **Model type:** Language model
- **Language(s) (NLP):** en
- **License:** apache-2.0
- **Parent Model:** More information needed
- **Resources for more information:** More information needed
  - [GitHub Repo](https://github.com/huggingface/huggingface_hub/tree/main)
  - [Associated Paper](https://www.semanticscholar.org/paper/The-What-If-Tool%3A-Interactive-Probing-of-Machine-Wexler-Pushkarna/1f6b9766374d14d81c225c2ced5bb02fe0bccd43)

## Uses

<!-- Address questions around how the model is intended to be used, including the foreseeable users of the model and those affected by the model. -->

### Direct Use

<!-- This section is for the model use without fine-tuning or plugging into a larger ecosystem/app. -->
<!-- If the user enters content, print that. If not, but they enter a task in the list, use that. If neither, say "more info needed." -->

This dataset that this model was trained on was orginially created to support the machine learning community in coductiong empirical analysis of ML algorithms. The Adult Data Set can be used in fairness-related studies that compare inequalities accross sex and race, based on people's annual incomes.

### Downstream Use [Optional]

<!-- This section is for the model use when fine-tuned for a task, or when plugged into a larger ecosystem/app -->
<!-- If the user enters content, print that. If not, but they enter a task in the list, use that. If neither, say "more info needed." -->

Base ethical classification models on results.

### Out-of-Scope Use

<!-- This section addresses misuse, malicious use, and uses that the model will not work well for. -->
<!-- If the user enters content, print that. If not, but they enter a task in the list, use that. If neither, say "more info needed." -->

The model is not intended to be used in production settings.

## Bias, Risks, and Limitations

<!-- This section is meant to convey both technical and sociotechnical limitations. -->

Significant research has explored bias and fairness issues with language models (see, e.g., [Sheng et al. (2021)](https://aclanthology.org/2021.acl-long.330.pdf) and [Bender et al. (2021)](https://dl.acm.org/doi/pdf/10.1145/3442188.3445922)). Predictions generated by the model may include disturbing and harmful stereotypes across protected classes; identity characteristics; and sensitive, social, and occupational groups.

### Recommendations

<!-- This section is meant to convey recommendations with respect to the bias, risk, and technical limitations. -->

As mentioned, some interventions may need to be performed to address the class imbalances in the dataset.

## Training Details

### Training Data

<!-- This should link to a Data Card, perhaps with a short stub of information on what the training data is all about as well as documentation related to data pre-processing or additional filtering. -->

The UCI Census Income Dataset is a widely used dataset for research purposes. It contains demographic information about individuals, as well as a binary label indicating whether or not the individual has an income of over $50,000. The dataset is used to train a model to predict this label based on the demographic information. The dataset is widely used for research purposes, but is not used in production settings. The dataset is not considered to be a sensitive dataset, and is not subject to privacy regulations. The dataset is not considered to be a sensitive dataset, and is not subject to privacy regulations.

### Training Procedure

<!-- This relates heavily to the Technical Specifications. Content here should link to that section when it is relevant to the training procedure. -->

#### Preprocessing

More information needed

#### Speeds, Sizes, Times

<!-- This section provides information about throughput, start/end time, checkpoint size if relevant, etc. -->

More information needed

## Evaluation

<!-- This section describes the evaluation protocols and provides the results. -->

### Testing Data, Factors & Metrics

#### Testing Data

<!-- This should link to a Data Card if possible. -->

https://archive.ics.uci.edu/ml/datasets/Census+Income

#### Factors

<!-- These are the things the evaluation is disaggregating by, e.g., subpopulations or domains. -->

Population of Race and Sex is imbalanced

#### Metrics

<!-- These are the evaluation metrics being used, ideally with a description of why. -->

binary_accuracy

### Results

Race | Other: 0.95
Race | Black: 0.89
Race | White: 0.8

## Model Examination

More information needed

## Environmental Impact

<!-- Total emissions (in grams of CO2eq) and additional considerations, such as electricity usage, go here. Edit the suggested text below accordingly -->

Carbon emissions can be estimated using the [Machine Learning Impact calculator](https://mlco2.github.io/impact#compute) presented in [Lacoste et al. (2019)](https://arxiv.org/abs/1910.09700).

- **Hardware Type:** A100+80GB
- **Hours used:** 24
- **Cloud Provider:** Scaleway
- **Compute Region:** Paris
- **Carbon Emitted:** 13

## Technical Specifications [optional]

### Model Architecture and Objective

Inference Endpoint

### Compute Infrastructure

Scaleway

#### Hardware

A100+80GB

#### Software

Python

## Citation

<!-- If there is a paper or blog post introducing the model, the APA and Bibtex information for that should go in this section. -->

**BibTeX:**

```bibtex
@article{Cabrera2023ZenoAI,
  title={Zeno: An Interactive Framework for Behavioral Evaluation of Machine Learning},
  author={{\&#39;A}ngel Alexander Cabrera and Erica Fu and Donald Bertucci and Kenneth Holstein and Ameet Talwalkar and Jason I. Hong and Adam Perer},
  journal={Proceedings of the 2023 CHI Conference on Human Factors in Computing Systems},
  year={2023},
  url={https://api.semanticscholar.org/CorpusID:256697444}
}
```

**APA:**

```apa
```

## Glossary [optional]

<!-- If relevant, include terms and calculations in this section that can help readers understand the model or model card. -->

LLM: Large Language Model

## More Information [optional]

No more information can be provided at this time.

## Model Card Authors [optional]

<!-- This section provides another layer of transparency and accountability. Whose views is this model card representing? How many voices were included in its construction? Etc. -->

Daniel Vorhauer <daniel.vorhauer@company.com>>, John Doe <john.doe@company.com>

## Model Card Contact

Daniel Vorhauer <daniel.vorhauer@company.com>

## How to Get Started with the Model

Use the code below to get started with the model.

```python

from huggingface_hub import hf_hub_download
import joblib

model = joblib.load(
    hf_hub_download("Census Income Classifier", "sklearn_model.joblib")
)

```
