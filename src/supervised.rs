use smartcore::dataset::*;
// DenseMatrix wrapper around Vec
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
// Imports for KNN classifier
use smartcore::math::distance::Distances;
use smartcore::neighbors::knn_classifier::*;
// Imports for Logistic Regression
use smartcore::linear::logistic_regression::LogisticRegression;
// Imports for Tree
use smartcore::tree::decision_tree_classifier::DecisionTreeClassifier;
// Imports for Random Classifier
use smartcore::ensemble::random_forest_classifier::RandomForestClassifier;
// Model performance
use smartcore::metrics::roc_auc_score;
use smartcore::model_selection::train_test_split;

pub fn breast_cancer() {
    // Load dataset
    let cancer_data = breast_cancer::load_dataset();
    // Transform dataset into a NxM matrix
    let x = DenseMatrix::from_array(
        cancer_data.num_samples,
        cancer_data.num_features,
        &cancer_data.data,
    );
    // These are our target class labels
    let y = cancer_data.target;

    // Split dataset into training/test (80%/20%)
    let (x_train, x_test, y_train, y_test) = train_test_split(&x, &y, 0.2);

    // KNN classifier
    let knn = KNNClassifier::fit(
        &x_train,
        &y_train,
        Distances::euclidian(),
        Default::default(),
    );
    let y_hat_knn = knn.predict(&x_test);

    // Logistic Regression
    let lr = LogisticRegression::fit(&x_train, &y_train);
    let y_hat_lr = lr.predict(&x_test);

    // Decision Tree
    let tree = DecisionTreeClassifier::fit(&x_train, &y_train, Default::default());
    let y_hat_tree = tree.predict(&x_test);

    // Random Forest
    let rf = RandomForestClassifier::fit(&x_train, &y_train, Default::default());
    let y_hat_rf = rf.predict(&x_test);

    // Calculate test error
    println!("AUC KNN: {}", roc_auc_score(&y_test, &y_hat_knn));
    println!(
        "AUC Logistic Regression: {}",
        roc_auc_score(&y_test, &y_hat_lr)
    );
    println!("AUC Decision Tree: {}", roc_auc_score(&y_test, &y_hat_tree));
    println!("AUC Random Forest: {}", roc_auc_score(&y_test, &y_hat_rf));
}