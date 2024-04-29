//! # My Callback System
//!
//! Ce module implémente un système de gestion de callbacks générique utilisant Rust.
//! Il fournit des structures et des traits pour enregistrer et exécuter des callbacks
//! avec des données associées. Les examples incluent comment configurer et utiliser ces callbacks.
//!
//! ## Fonctionnalités
//!
//! - `CallbackData`: Trait servant de base pour les types pouvant être utilisés comme données dans des callbacks.
//! - `MyCallbackData`: Structure concrète implémentant `CallbackData`, stockant des références à des données.
//! - `MyCallback`: Structure générique pour gérer des callbacks.
//! - `MyTrait`: Trait pour les structures désirant implémenter un système de callback.
//! - `MyStruct`: Implémentation d'une structure utilisant `MyTrait` et gérant plusieurs callbacks.

/// Définition d'un trait vide nommé `CallbackData`. Les traits peuvent définir des comportements communs que divers types peuvent implémenter.
trait CallbackData {}

/// Représente des données de callback contenant une référence à un slice de bytes.
///
/// Cette structure stocke une référence à des données qui doivent rester valides pendant la durée de vie de l'objet.
///
/// # Examples
///
/// ```
/// let data = vec![1, 2, 3, 4];
/// let callback_data = MyCallbackData { data: &data };
/// ```
#[derive(Debug)]
struct MyCallbackData<'a> {
    data: &'a [u8],
}

/// Implémentation du trait `CallbackData` pour `MyCallbackData`. Ceci permet à `MyCallbackData` d'être utilisé là où `CallbackData` est requis.
impl<'a> CallbackData for MyCallbackData<'a> {}

/// Générique qui permet de gérer un callback.
///
/// `MyCallback` est une structure qui encapsule une fonction (ou closure) qui sera appelée avec une référence à une donnée de type `T`.
///
/// # Type Parameters
///
/// - `T`: Le type des données de callback. `T` doit implémenter `CallbackData`.
///
/// # Examples
///
/// ```
/// let callback = MyCallback {
///     callback: Box::new(|data: &MyCallbackData| println!("Data: {:?}", data)),
/// };
/// ```
struct MyCallback<T: CallbackData> {
    callback: Box<dyn Fn(&T)>, // Le champ `callback` est une boîte contenant une fonction anonyme qui prend une référence à un type `T`.
}

/// `MyTrait` définit les comportements pour les structures qui veulent implémenter un mécanisme de callback.
///
/// Ce trait permet de configurer un ou plusieurs callbacks et de les exécuter.
///
/// # Examples
///
/// ```
/// struct ExampleStruct {
///     callbacks: Vec<MyCallback<MyCallbackData<'static>>>,
///     data: &'static [u8; 3],
/// }
///
/// impl MyTrait<'static, MyCallbackData<'static>> for ExampleStruct {
///     fn set_callback(&mut self, cb: MyCallback<MyCallbackData<'static>>) {
///         self.callbacks.push(cb);
///     }
///
///     fn do_something(&self) {
///         for cb in &self.callbacks {
///             let cb_data = MyCallbackData { data: self.data };
///             (cb.callback)(&cb_data);
///         }
///     }
/// }
/// ```
trait MyTrait<'a, T: CallbackData> {
    fn set_callback(&mut self, cb: MyCallback<T>); // Méthode pour ajouter un callback.
    fn do_something(&self); // Méthode abstraite pour effectuer une action, non définie ici.
}

/// `MyStruct` est une structure générique qui utilise `CallbackData` pour gérer une série de callbacks et des données associées.
///
/// # Type Parameters
///
/// - `T`: Le type des données de callback. `T` doit implémenter `CallbackData`.
/// - `'a`: La durée de vie des références aux données.
///
/// # Fields
///
/// - `callbacks`: Un vecteur de `MyCallback<T>` pour stocker les fonctions de rappel.
/// - `data`: Une référence à un tableau fixe de trois éléments de type byte.
///
/// # Examples
///
/// ```
/// let data = &[1, 2, 3];
/// let mut my_struct = MyStruct {
///     callbacks: Vec::new(),
///     data: data,
/// };
/// my_struct.set_callback(MyCallback {
///     callback: Box::new(|data: &MyCallbackData| println!("Data: {:?}", data)),
/// });
/// my_struct.do_something();
/// ```
struct MyStruct<'a, T: CallbackData> {
    callbacks: Vec<MyCallback<T>>, // Vecteur de callbacks de type `T`.
    data: &'a [u8; 3],             // Un tableau fixe de trois éléments de type byte.
}

/// Implémentation du trait `MyTrait` pour `MyStruct` utilisant `MyCallbackData` avec une lifetime.
impl<'a> MyTrait<'a, MyCallbackData<'a>> for MyStruct<'a, MyCallbackData<'a>> {
    // Ajoute un `MyCallback` au vecteur de callbacks.
    fn set_callback(&mut self, cb: MyCallback<MyCallbackData<'a>>) {
        self.callbacks.push(cb);
    }

    // Itère sur chaque callback dans le vecteur et les exécute avec les données actuelles.
    fn do_something(&self) {
        for cb in &self.callbacks {
            let cb_data = MyCallbackData {
                data: self.data, // Crée un `MyCallbackData` avec une référence aux données de `MyStruct`.
            };

            (cb.callback)(&cb_data); // Exécute le callback avec `cb_data`.
            process_data(cb_data.data); // Utilisez 'data' ici
        }
    }
}

/// Fonction pour traiter des données.
///
/// Cette fonction sert d'exemple pour montrer comment les données peuvent être traitées.
///
/// # Arguments
///
/// * `data` - Une référence à un slice de bytes à traiter.
///
/// # Examples
///
/// ```
/// process_data(&[1, 2, 3, 4]);
/// ```
fn process_data(data: &[u8]) {
    // Imaginez que vous faites quelque chose d'utile avec les données ici
    println!("Processing data: {:?}", data);
}

/// Fonction principale qui s'exécute lorsque le programme est lancé.
fn main() {
    let mut s = MyStruct {
        callbacks: Vec::new(), // Initialise un vecteur vide de callbacks.
        data: &[1, 2, 3],      // Initialise les données avec les valeurs 1, 2 et 3.
    };

    // Ajoute un callback à `s` qui imprime les données passées.
    s.set_callback(MyCallback {
        // `Box::new` crée une nouvelle boîte (Box) qui alloue dynamiquement en mémoire. Ici, elle contient une closure (fonction anonyme).
        // Cette closure prend un argument `data` qui est une référence à `MyCallbackData`.
        callback: Box::new(|data: &MyCallbackData| {
            // La closure imprime le contenu de `data` à l'écran.
            // `{:?}` est un spécificateur de format utilisé pour afficher les données dérivées de `Debug`.
            println!("Callback called with data {:?}", data);
        }),
    });
    // Appelle `do_something` sur `s`, ce qui exécute tous les callbacks ajoutés.
    s.do_something();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Teste la création de `MyCallbackData` avec une référence valide.
    #[test]
    fn test_callback_data_creation() {
        let data = vec![1, 2, 3, 4];
        let callback_data = MyCallbackData { data: &data };
        assert_eq!(callback_data.data, &[1, 2, 3, 4]);
    }

    /// Teste la fonctionnalité `set_callback` pour s'assurer qu'elle ajoute correctement un callback au vecteur.
    #[test]
    fn test_set_callback() {
        let data = &[1, 2, 3];
        let mut my_struct = MyStruct {
            callbacks: Vec::new(),
            data: data,
        };

        my_struct.set_callback(MyCallback {
            callback: Box::new(|_data: &MyCallbackData| {}),
        });

        assert_eq!(my_struct.callbacks.len(), 1);
    }

    /// Teste la fonction `process_data` pour vérifier qu'elle traite les données correctement.
    #[test]
    fn test_process_data() {
        let data = &[1, 2, 3, 4];
        process_data(data); // Supposé imprimer "Processing data: [1, 2, 3, 4]"

        // Ce test est trivial car il s'attend à ce que `process_data` fonctionne.
        // En pratique, vous voudrez peut-être vérifier l'état ou le comportement
        // (par exemple, en utilisant un mock ou en vérifiant les sorties/loggings).
    }
}
