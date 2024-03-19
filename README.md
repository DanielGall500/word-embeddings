# Word Embeddings
The human brain is built to quickly understand the morphological, syntactic and phonological properties of words. In order for a language model to reach any form of similar understanding, words must be converted to an alternative numerical representation - one which can be used to encode similarity to others, the context in which it occurs, and it other properties which tell a model about its semantic makeup. These word embeddings are at the core of state-of-the-art NLP tools.

## Types of Word-Embedding Algorithms

#### Traditional Methods
* One Hot Encoding - A sparse binary matrix consisting of a column for each unique word and a value 0/1 depending on whether that word occurs in a sample.
* Bag Of Words - An extension on One Hot Encoding which counts the number of times a word occurs in a sample and uses this value for the corresponding position in the matrix for that word.
* TF-IDF (Term Frequency - Inverse Document Frequency) - Term frequency refers to the number of times a term occurs in a sample, and Inverse Document Frequency refers to how much information that term actually provides us with about the type of document. For instance, "okay" provides us with very little information, while "stock market" provides us with a lot of information. The high frequency of a word within one document combined with the low frequency of that term across all documents generally will indicate that it is a particularly theme-revealing term in the document.

Other methods such as GloVe create a semantic vector space, that is vector space in which terms are assigned a real-valued number and words with a closer number are deemed more semantically related. Trains only on non-zero elements in a co-occurrence matrix, rather than on the entire sparse matrix or individual context windows in a large corpus. In the original paper for GloVe, Pennington et al. specify two main model families for learning word vectors:
1. Global matrix factorisation methods (LSA).
2. Local Context Window Methods (Skip-gram)
