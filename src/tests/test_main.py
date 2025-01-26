import unittest
from unittest.mock import patch, MagicMock, call
from src.main import divide_photos, select_raf_from_jpg  # Assumendo che il file si chiami script.py


class TestPhotoOrganizer(unittest.TestCase):
    @patch('os.chdir')
    @patch('os.rename')
    @patch('os.path.isfile')
    @patch('os.listdir')
    @patch('os.makedirs')
    def test_divide_photos(self, mock_makedirs, mock_listdir, mock_isfile, mock_rename, mock_chdir):
        mock_listdir.return_value = ['photo1.JPG', 'photo1.RAF', 'doc.txt']
        mock_isfile.side_effect = lambda x: x in ['photo1.JPG', 'photo1.RAF', 'doc.txt']
        mock_chdir.return_value = None

        # Esegui la funzione
        divide_photos('test_path')

        mock_makedirs.assert_has_calls([
            call('jpg', exist_ok=True),
            call('raf', exist_ok=True),
            call('jpg/selected', exist_ok=True),
            call('raf/selected', exist_ok=True),
        ], any_order=True)

        mock_rename.assert_has_calls([
            call('photo1.JPG', 'jpg/photo1.JPG'),
            call('photo1.RAF', 'raf/photo1.RAF'),
        ], any_order=True)

    @patch('os.chdir')
    @patch('os.rename')
    @patch('os.path.exists')
    @patch('os.path.isdir')
    @patch('os.listdir')
    @patch('os.makedirs')
    def test_select_raf_from_jpg(self, mock_makedirs, mock_listdir, mock_isdir, mock_exists, mock_rename, mock_chdir):
        mock_isdir.return_value = True
        mock_listdir.return_value = ['photo1.JPG']
        mock_exists.side_effect = lambda x: x in ['raf/photo1.RAF']
        mock_chdir.return_value = None

        select_raf_from_jpg('test_path')

        mock_makedirs.assert_called_once_with('raf/selected', exist_ok=True)

        mock_rename.assert_called_once_with('raf/photo1.RAF', 'raf/selected/photo1.RAF')


if __name__ == '__main__':
    unittest.main()