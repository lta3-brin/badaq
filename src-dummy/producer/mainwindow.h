#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QTcpSocket>
#include <QString>
#include <QDate>
#include <QTime>
#include <QHostAddress>
#include <QDataStream>
#include <QFile>
#include <QFileDialog>
#include <QTextStream>
#include <QStringList>

QT_BEGIN_NAMESPACE
namespace Ui {
class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void Disconnect();
    void WriteMessage(QString Mssg);
    void on_Btn_Open_Conn_clicked();
    void on_Btn_Close_Conn_clicked();
    void on_Btn_Stream_clicked();
    void on_Btn_Open_file_clicked();

private:
    Ui::MainWindow *ui;
    QTcpSocket* Socket;
    int Phase;
    int idLine;
    int Length;
    int Seq;
    QStringList List;
};
#endif // MAINWINDOW_H
